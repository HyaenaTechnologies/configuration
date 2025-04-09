use std::{
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
};

// Print Help Command Output
pub fn print_help_message() -> ExitCode {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Systsem Update Daemon").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Commands:					Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "apt-upgrade               APT Upgrade").unwrap();
    writeln!(
        standard_output,
        "dnf-release               DNF System Release Upgrade"
    )
    .unwrap();
    writeln!(standard_output, "dnf-upgrade               DNF Upgrade").unwrap();
    writeln!(
        standard_output,
        "help                      Print List of Commands and Flags"
    )
    .unwrap();
    writeln!(standard_output, "snap-refresh              Snap Refresh").unwrap();
    writeln!(
        standard_output,
        "ubuntu-release            Ubuntu System Releade Upgrade"
    )
    .unwrap();
    writeln!(
        standard_output,
        "version                   Print Version Number"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Flags:				    Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "--au                      APT Upgrade").unwrap();
    writeln!(
        standard_output,
        "--dr                      DNF System Release Upgrade"
    )
    .unwrap();
    writeln!(standard_output, "--du                      DNF Upgrade").unwrap();
    writeln!(
        standard_output,
        "--h                       Print List of Commands and Flags"
    )
    .unwrap();
    writeln!(standard_output, "--sr                      Snap Refresh").unwrap();
    writeln!(
        standard_output,
        "--ur                      Ubuntu System Releade Upgrade"
    )
    .unwrap();
    writeln!(
        standard_output,
        "--v                       Print Version Number"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();

    return ExitCode::SUCCESS;
}
