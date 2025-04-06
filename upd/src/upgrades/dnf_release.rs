use std::{
    io::Error,
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
};

// DNF System Release Upgrade
pub fn release_dnf() -> ExitCode {
    let dnf_release: Result<Output, Error> = Command::new("dnf")
        .arg("system-upgrade")
        .arg("download")
        .arg("--releasever=41")
        .output();

    match dnf_release {
        Ok(release) => {
            println!("Command Output: {:#?}", String::from_utf8(release.stdout));
            println!("Status: {}", release.status);
        }
        Err(error) => {
            eprintln!("System Release Upgrade Error for DNF {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
