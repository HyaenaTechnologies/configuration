use std::{
    io::{Error, Stdout, Write, stdout},
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// DNF System Release Upgrade
pub fn release_dnf() -> ExitCode {
    let dnf_release: Result<Output, Error> = Command::new("dnf")
        .arg("system-upgrade")
        .arg("download")
        .arg("--releasever=41")
        .output();
    let mut standard_output: Stdout = stdout();

    match dnf_release {
        Ok(release) => {
            standard_output.write_all(&release.stdout).unwrap();
            println!("Status: {}", release.status);
        }
        Err(error) => {
            eprintln!("Error Executing DNF System Release Upgrade: {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
