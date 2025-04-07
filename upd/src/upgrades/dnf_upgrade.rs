use std::{
    io::{Error, StdoutLock, Write, stdout},
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// DNF Upgrade
pub fn upgrade_dnf() -> ExitCode {
    let dnf_upgrade: Result<Output, Error> = Command::new("dnf").arg("-y").arg("upgrade").output();
    let mut standard_output: StdoutLock = stdout().lock();

    match dnf_upgrade {
        Ok(upgrade) => {
            standard_output.write_all(&upgrade.stdout).unwrap();
            println!("Status: {}", upgrade.status);
        }
        Err(error) => {
            eprintln!("Error Executing DNF Upgrade: {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
