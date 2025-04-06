use std::{
    io::{Error, Stdout, Write, stdout},
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Ubuntu System Release Upgrade
pub fn release_ubuntu() -> ExitCode {
    let ubuntu_release: Result<Output, Error> = Command::new("do-release-upgrade").output();
    let mut standard_output: Stdout = stdout();

    match ubuntu_release {
        Ok(release) => {
            standard_output.write_all(&release.stdout).unwrap();
            println!("Status: {}", release.status);
        }
        Err(error) => {
            eprintln!("System Release Upgrade Error for Ubuntu {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
