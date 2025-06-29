use std::{
    io::{Error, StdoutLock, Write, stdout},
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// APT Upgrade
pub fn upgrade_apt() -> ExitCode {
    let apt_update: Result<Output, Error> = Command::new("apt").arg("update").output();
    let apt_upgrade: Result<Output, Error> =
        Command::new("apt").arg("-y").arg("full-upgrade").output();
    let mut standard_output: StdoutLock = stdout().lock();

    match apt_update {
        Ok(update) => {
            standard_output.write_all(&update.stdout).unwrap();
            writeln!(standard_output, "Status: {}", update.status).unwrap();
        }
        Err(error) => {
            eprintln!("Error Executing APT Update: {}", error);
            return ExitCode::FAILURE;
        }
    };

    match apt_upgrade {
        Ok(upgrade) => {
            standard_output.write_all(&upgrade.stdout).unwrap();
            writeln!(standard_output, "Status: {}", upgrade.status).unwrap();
        }
        Err(error) => {
            eprint!("Error Executing APT Upgrade: {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
