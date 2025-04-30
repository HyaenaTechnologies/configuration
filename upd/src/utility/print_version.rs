use std::{
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
    string::String,
};

// Semantic Version Number Definition
type SemanticVersionNumber = String;

// Print Version Number
pub fn print_version_number() -> ExitCode {
    let mut standard_output: StdoutLock = stdout().lock();
    let version_number: SemanticVersionNumber = "0.2.0".to_string();

    writeln!(standard_output, "Systsem Update Daemon").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Version Number:		{}", version_number).unwrap();

    return ExitCode::SUCCESS;
}
