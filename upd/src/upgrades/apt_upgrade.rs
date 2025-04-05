use std::{
    io::Error,
    process::{Command, ExitCode, Output},
    result::Result,
    result::Result::Err,
    result::Result::Ok,
    string::String,
};

// APT Upgrade
pub fn upgrade_apt() -> ExitCode {
    let apt_update: Result<Output, Error> = Command::new("apt").arg("update").output();
    match apt_update {
        Ok(update) => {
            println!("Command Output: {:#?}", String::from_utf8(update.stdout));
            println!("Status: {:#?}", update.status);
            println!("Error (If Error): {:#?}", String::from_utf8(update.stderr));
        }
        Err(error) => {
            eprintln!("Error Updating APT {}", error);
            return ExitCode::FAILURE;
        }
    };

    let apt_upgrade: Result<Output, Error> =
        Command::new("apt").arg("-y").arg("full-upgrade").output();

    match apt_upgrade {
        Ok(upgrade) => {
            println!("Command Output: {:#?}", String::from_utf8(upgrade.stdout));
            println!("Status: {:#?}", upgrade.status);
            println!("Error (If Error): {:#?}", String::from_utf8(upgrade.stderr));
        }
        Err(error) => {
            eprintln!("Error Upgrading APT {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
