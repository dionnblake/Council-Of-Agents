# Parallel Security Specification

## Purpose

This directory defines threat models, adversarial prompt corpora, and safety acceptance cases for Council of Agents. It is a non-runtime QA package.

## Ownership

This directory owns security reasoning and test design for the Council boundary. It does not own production process control, snapshot code, provider adapters, credentials, or executable exploit tooling.

## Local Contracts

- Use synthetic markers and harmless fixture actions only.
- Cover malicious repository content, provider behavior, prompt injection, evidence poisoning, and master-prompt boundary failures.
- Define expected detection, blocking state, audit event, and safe recovery for each case.
- Fail closed when a security invariant is unverified.

## Verification

Review that every case has an ID, threat actor or source, precondition, action, expected result, and retained evidence. Do not claim a runtime pass from this documentation alone.

## Child DOX Index

This directory has no child `AGENTS.md` files.
