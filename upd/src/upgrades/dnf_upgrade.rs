use std::{
    io::Error,
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
};

// DNF Upgrade
pub fn upgrade_dnf() -> ExitCode {
    let dnf_upgrade: Result<Output, Error> = Command::new("dnf").arg("-y").arg("upgrade").output();

    match dnf_upgrade {
        Ok(upgrade) => {
            println!("Command Output: {:#?}", String::from_utf8(upgrade.stdout));
            println!("Status: {}", upgrade.status);
        }
        Err(error) => {
            eprintln!("Error Upgrading DNF {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
