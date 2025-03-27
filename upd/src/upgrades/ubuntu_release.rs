use std::process::{Command, ExitCode, Output};

// Ubuntu System Release Upgrade
pub fn release_ubuntu() -> ExitCode {
    let dnf_upgrade: Output = Command::new("do-release-upgrade")
        .output()
        .expect("Ubuntu Release System Upgrade Failed");

    println!(
        "Command Output: {:#?}",
        String::from_utf8(dnf_upgrade.stdout)
    );
    println!("Status: {:#?}", dnf_upgrade.status);
    println!(
        "Error (If Error): {:#?}",
        String::from_utf8(dnf_upgrade.stderr)
    );

    return ExitCode::SUCCESS;
}
