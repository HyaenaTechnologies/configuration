use std::{
    process::{Command, ExitCode, Output},
    string::String,
};

// APT Upgrade
pub fn upgrade_apt() -> ExitCode {
    let apt_update: Output = Command::new("apt")
        .arg("update")
        .output()
        .expect("APT Update Failed");

    println!(
        "Command Output: {:#?}",
        String::from_utf8(apt_update.stdout)
    );
    println!("Status: {:#?}", apt_update.status);
    println!(
        "Error (If Error): {:#?}",
        String::from_utf8(apt_update.stderr)
    );

    let apt_upgrade: Output = Command::new("apt")
        .arg("-y")
        .arg("full-upgrade")
        .output()
        .expect("APT FUll Upgrade Failed");

    println!(
        "Command Output: {:#?}",
        String::from_utf8(apt_upgrade.stdout)
    );
    println!("Status: {:#?}", apt_upgrade.status);
    println!(
        "Error (If Error): {:#?}",
        String::from_utf8(apt_upgrade.stderr)
    );

    return ExitCode::SUCCESS;
}
