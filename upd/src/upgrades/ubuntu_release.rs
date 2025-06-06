use std::{
    io::{Error, StdoutLock, Write, stdout},
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Ubuntu System Release Upgrade
pub fn release_ubuntu() -> ExitCode {
    let ubuntu_release: Result<Output, Error> = Command::new("do-release-upgrade").output();
    let mut standard_output: StdoutLock = stdout().lock();

    match ubuntu_release {
        Ok(release) => {
            standard_output.write_all(&release.stdout).unwrap();
            writeln!(standard_output, "Status: {}", release.status).unwrap();
        }
        Err(error) => {
            eprintln!("Error Executing Ubuntu System Release Upgrade: {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
