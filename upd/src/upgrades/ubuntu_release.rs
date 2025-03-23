use std::process::Command;

// Ubuntu System Release Upgrade
pub fn release_ubuntu() -> () {
    let dnf_upgrade: Output = Command::new("do-release-upgrade")
    .output()
    .expect("Ubuntu Release System Upgrade Failed");

    println!("Command Output: {:#?}", String::from_utf8(dnf_upgrade.stdout));
    println!("Status: {:#?}", dnf_upgrade.status);
    println!("Error (If Error): {:#?}", String::from_utf8(dnf_upgrade.stderr));
}
