use std::{
    io::Error,
    process::{Command, ExitCode, Output},
    result::Result,
    result::Result::Err,
    result::Result::Ok,
    string::String,
};

// DNF System Release Upgrade
pub fn release_dnf() -> ExitCode {
    let dnf_release: Output = Command::new("dnf")
        .arg("-y")
        .arg("system-upgrade")
        .arg("download")
        .arg("--releasever=41")
        .output()
        .expect("DNF System Release Upgrade Failed");

    println!(
        "Command Output: {:#?}",
        String::from_utf8(dnf_release.stdout)
    );
    println!("Status: {:#?}", dnf_release.status);
    println!(
        "Error (If Error): {:#?}",
        String::from_utf8(dnf_release.stderr)
    );

    return ExitCode::SUCCESS;
}
