use clap::{Parser, Subcommand};
use council_core::{
    ContextPacket, Database, EvidenceIndex, Intake, ModelSelection, ProviderKind, ProviderRegistry,
    SnapshotBuilder, SnapshotRequest, billing_environment_status, compile_decision_record,
    compile_master_prompt, validate_intake,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "council-cli",
    version,
    about = "Headless Council of Agents controller"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Providers,
    Init {
        #[arg(long, default_value = "council.sqlite3")]
        database: PathBuf,
    },
    ValidateIntake {
        input: PathBuf,
    },
    Snapshot {
        source: PathBuf,
        destination: PathBuf,
        #[arg(long)]
        id: String,
    },
    VerifyEvidence {
        root: PathBuf,
        citation: String,
        #[arg(long)]
        expected: Option<String>,
    },
    Demo {
        #[arg(long, default_value = "council-demo")]
        output: PathBuf,
    },
    Compile {
        decision: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Providers => {
            let registry = ProviderRegistry::defaults();
            let billing = billing_environment_status();
            let billing_state = if billing.values().any(|present| *present) {
                "BLOCKED_ENVIRONMENT_VARIABLE_PRESENT"
            } else {
                "SUBSCRIPTION_ONLY_ENVIRONMENT"
            };
            println!("BILLING\t{billing_state}");
            for config in registry.all() {
                let preflight = registry.preflight(&config.provider);
                println!(
                    "{}\tmodel={}\tcertification={:?}\tpreflight={}",
                    config.provider.display_name(),
                    config.model_default,
                    config.certification,
                    match preflight {
                        Ok(()) => "READY".to_string(),
                        Err(error) => format!("NOT_READY: {error}"),
                    }
                );
            }
        }
        Command::Init { database } => {
            Database::open(&database)?;
            println!("Initialized {}", database.display());
        }
        Command::ValidateIntake { input } => {
            let value: Intake = serde_json::from_slice(&fs::read(&input)?)?;
            match validate_intake(&value) {
                Ok(()) => println!("INTAKE_VALID"),
                Err(errors) => {
                    println!("INTAKE_INVALID");
                    for error in errors {
                        println!("- {error}");
                    }
                    std::process::exit(2);
                }
            }
        }
        Command::Snapshot {
            source,
            destination,
            id,
        } => {
            let manifest = SnapshotBuilder::new().build(&SnapshotRequest {
                source_root: source,
                destination_root: destination,
                snapshot_id: id,
            })?;
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
        Command::VerifyEvidence {
            root,
            citation,
            expected,
        } => {
            let index = EvidenceIndex::build(root)?;
            let evidence = index.verify(&citation, expected.as_deref());
            println!("{}", serde_json::to_string_pretty(&evidence)?);
        }
        Command::Demo { output } => run_demo(&output)?,
        Command::Compile { decision, output } => {
            let record = serde_json::from_slice(&fs::read(&decision)?)?;
            fs::create_dir_all(&output)?;
            fs::write(
                output.join("master-prompt.md"),
                compile_master_prompt(&record),
            )?;
            fs::write(
                output.join("decision-record.md"),
                compile_decision_record(&record),
            )?;
            println!("COMPILED {}", output.display());
        }
    }
    Ok(())
}

fn run_demo(output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output)?;
    let database_path = output.join("council.sqlite3");
    let database = Database::open(&database_path)?;
    let intake = Intake {
        question: "Should this local-first Windows tool use SQLite or PostgreSQL?".to_string(),
        mode: council_core::DebateMode::Compare,
        options: vec!["SQLite".to_string(), "PostgreSQL".to_string()],
        product_type: council_core::ProductType::Windows,
        decision_type: council_core::DecisionType::Database,
        hard_constraints: vec![
            "Local-first".to_string(),
            "No hosted dependency".to_string(),
        ],
        priority: "Simplest to maintain".to_string(),
        current_leaning: None,
        current_leaning_reason: None,
        repository: None,
    };
    let models = BTreeMap::from([
        (
            ProviderKind::Claude,
            ModelSelection::requested("claude-haiku-4-5-20251001"),
        ),
        (
            ProviderKind::Antigravity,
            ModelSelection::requested("gemini-3.7-flash-low"),
        ),
        (
            ProviderKind::CodexWsl,
            ModelSelection::requested("gpt-5.6-luna"),
        ),
    ]);
    let debate = council_core::Debate::new(intake, models);
    database.create_debate(&debate)?;
    database.transition_debate(&debate.id, council_core::DebateEvent::PreflightPassed)?;
    database.transition_debate(&debate.id, council_core::DebateEvent::SnapshotStarted)?;
    database.transition_debate(&debate.id, council_core::DebateEvent::SnapshotReady)?;
    database.transition_debate(&debate.id, council_core::DebateEvent::OpeningStarted)?;

    let packet = ContextPacket::new(
        &debate.id,
        "demo-turn",
        ProviderKind::CodexWsl,
        council_core::POSITION_SCHEMA_VERSION,
        "Question: SQLite or PostgreSQL?\nConstraints: local-first, no hosted dependency.\n",
    );
    let packet_directory = output.join("packets");
    let written = packet.write_sealed(&packet_directory)?;
    fs::write(
        output.join("demo-packet.json"),
        serde_json::to_vec_pretty(&written)?,
    )?;
    println!("DEMO_READY {}", output.display());
    Ok(())
}
