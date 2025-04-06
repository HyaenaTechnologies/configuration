use std::{
    io::Error,
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
};

// Ubuntu System Release Upgrade
pub fn release_ubuntu() -> ExitCode {
    let ubuntu_release: Result<Output, Error> = Command::new("do-release-upgrade").output();

    match ubuntu_release {
        Ok(release) => {
            println!("Command Output: {:#?}", String::from_utf8(release.stdout));
            println!("Status: {}", release.status);
        }
        Err(error) => {
            eprintln!("System Release Upgrade Error for Ubuntu {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
