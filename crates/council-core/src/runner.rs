use crate::providers::{CommandInvocation, CommandSpec};
use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    pub exit_code: Option<i32>,
    pub wall_ms: u128,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
    pub cancellation_fallback_ran: bool,
}

#[derive(Debug, Error)]
pub enum ProcessRunnerError {
    #[error("could not spawn provider process: {0}")]
    Spawn(#[from] std::io::Error),
    #[error("provider process output reader failed")]
    OutputReader,
    #[error("Windows process containment job could not be established: {0}")]
    JobContainment(String),
}

#[derive(Debug, Clone)]
pub struct ProcessRunner {
    pub timeout_grace: Duration,
    pub poll_interval: Duration,
}

impl Default for ProcessRunner {
    fn default() -> Self {
        Self {
            timeout_grace: Duration::from_millis(500),
            poll_interval: Duration::from_millis(20),
        }
    }
}

impl ProcessRunner {
    pub fn run(
        &self,
        specification: &CommandSpec,
        prompt: &str,
    ) -> Result<ProcessResult, ProcessRunnerError> {
        let started = Instant::now();
        let mut command = Command::new(&specification.program);
        command
            .args(&specification.args)
            .current_dir(&specification.working_directory)
            .env_clear()
            .envs(&specification.environment)
            .stdin(if specification.prompt_via_stdin {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn()?;
        #[cfg(windows)]
        let _job = if specification.windows_job_containment {
            Some(attach_kill_on_close_job(&child)?)
        } else {
            None
        };

        if specification.prompt_via_stdin {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(prompt.as_bytes())?;
                stdin.flush()?;
            }
        }

        let stdout_handle = spawn_reader(child.stdout.take());
        let stderr_handle = spawn_reader(child.stderr.take());
        let mut timed_out = false;
        let exit_code = loop {
            if let Some(status) = child.try_wait()? {
                break status.code();
            }
            if started.elapsed() >= Duration::from_millis(specification.timeout_ms) {
                timed_out = true;
                thread::sleep(self.timeout_grace);
                if child.try_wait()?.is_none() {
                    child.kill()?;
                }
                break child.wait()?.code();
            }
            thread::sleep(self.poll_interval);
        };

        let stdout = stdout_handle
            .join()
            .map_err(|_| ProcessRunnerError::OutputReader)??;
        let stderr = stderr_handle
            .join()
            .map_err(|_| ProcessRunnerError::OutputReader)??;
        let cancellation_fallback_ran = timed_out
            && specification
                .kill_fallback
                .as_ref()
                .map(run_cancellation_fallback)
                .unwrap_or(false);

        Ok(ProcessResult {
            exit_code,
            wall_ms: started.elapsed().as_millis(),
            stdout: String::from_utf8_lossy(&stdout).to_string(),
            stderr: String::from_utf8_lossy(&stderr).to_string(),
            timed_out,
            cancellation_fallback_ran,
        })
    }
}

fn spawn_reader(
    pipe: Option<impl Read + Send + 'static>,
) -> thread::JoinHandle<Result<Vec<u8>, ProcessRunnerError>> {
    thread::spawn(move || {
        let Some(mut pipe) = pipe else {
            return Ok(Vec::new());
        };
        let mut bytes = Vec::new();
        pipe.read_to_end(&mut bytes)
            .map(|_| bytes)
            .map_err(|_| ProcessRunnerError::OutputReader)
    })
}

fn run_cancellation_fallback(invocation: &CommandInvocation) -> bool {
    Command::new(&invocation.program)
        .args(&invocation.args)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(windows)]
fn attach_kill_on_close_job(child: &Child) -> Result<JobHandle, ProcessRunnerError> {
    use std::mem::{size_of, zeroed};
    use std::os::windows::io::AsRawHandle;
    use std::ptr::null_mut;
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
        SetInformationJobObject,
    };

    let job = unsafe { CreateJobObjectW(null_mut(), null_mut()) };
    if job.is_null() {
        return Err(ProcessRunnerError::JobContainment(
            "CreateJobObjectW returned null".to_string(),
        ));
    }
    let mut limits: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { zeroed() };
    limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    let configured = unsafe {
        SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            &mut limits as *mut _ as *mut _,
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        )
    };
    if configured == 0 {
        unsafe { CloseHandle(job) };
        return Err(ProcessRunnerError::JobContainment(
            "SetInformationJobObject failed".to_string(),
        ));
    }
    let assigned = unsafe { AssignProcessToJobObject(job, child.as_raw_handle() as HANDLE) };
    if assigned == 0 {
        unsafe { CloseHandle(job) };
        return Err(ProcessRunnerError::JobContainment(
            "AssignProcessToJobObject failed".to_string(),
        ));
    }
    Ok(JobHandle(job))
}

#[cfg(windows)]
struct JobHandle(windows_sys::Win32::Foundation::HANDLE);

#[cfg(windows)]
impl Drop for JobHandle {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::CommandSpec;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    #[test]
    fn runner_spec_keeps_timeout_and_fallback_metadata_typed() {
        let spec = CommandSpec {
            program: PathBuf::from("provider"),
            args: Vec::new(),
            environment: BTreeMap::new(),
            working_directory: PathBuf::from("."),
            prompt_via_stdin: false,
            windows_job_containment: true,
            timeout_ms: 100,
            kill_fallback: None,
        };
        assert_eq!(spec.timeout_ms, 100);
        assert!(!spec.prompt_via_stdin);
    }
}
