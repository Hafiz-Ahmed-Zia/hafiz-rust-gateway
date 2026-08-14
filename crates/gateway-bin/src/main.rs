use std::{env, process::ExitCode};

use hafiz_gateway_core::FailureClass;
use hafiz_gateway_testkit::named_scenario;

const HELP: &str = "Hafiz Rust Gateway research CLI

Usage:
  hafiz-gateway version
  hafiz-gateway explain-failure <reason-code>
  hafiz-gateway simulate <complete|precommit-disconnect|partial-disconnect>

This pre-alpha binary demonstrates failure contracts only. It is not an HTTP gateway yet.";

fn main() -> ExitCode {
    let mut arguments = env::args().skip(1);
    match arguments.next().as_deref() {
        Some("version") => {
            println!("hafiz-gateway {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some("explain-failure") => explain_failure(arguments.next()),
        Some("simulate") => simulate(arguments.next()),
        Some("help" | "--help" | "-h") | None => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        Some(command) => {
            eprintln!("unknown command: {command}\n\n{HELP}");
            ExitCode::from(2)
        }
    }
}

fn explain_failure(value: Option<String>) -> ExitCode {
    let Some(value) = value else {
        eprintln!("missing failure reason code");
        return ExitCode::from(2);
    };
    let Ok(failure) = value.parse::<FailureClass>() else {
        eprintln!("unknown failure reason code: {value}");
        return ExitCode::from(2);
    };

    println!("reason={}", failure.code());
    println!(
        "retryable_before_commit={}",
        failure.retryable_before_commit()
    );
    println!("affects_target_health={}", failure.affects_target_health());
    ExitCode::SUCCESS
}

fn simulate(value: Option<String>) -> ExitCode {
    let Some(name) = value else {
        eprintln!("missing scenario name");
        return ExitCode::from(2);
    };
    let Some(scenario) = named_scenario(&name) else {
        eprintln!("unknown scenario: {name}");
        return ExitCode::from(2);
    };
    let Ok(report) = scenario.run() else {
        eprintln!("scenario failed contract validation: {name}");
        return ExitCode::from(1);
    };

    println!("scenario={}", report.name);
    println!("terminal_reason={}", report.terminal_reason.code());
    println!("committed={}", report.committed);
    println!("forwarded_events={}", report.forwarded_events);
    println!("partial={}", report.partial);
    println!(
        "transparent_recovery_candidate={}",
        report.transparent_recovery_candidate
    );
    ExitCode::SUCCESS
}
