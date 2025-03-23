use std::process::Command;

// Snap Refresh
pub fn refresh_snap() -> () {
    let dnf_release: Output = Command::new("snap")
    .arg("refresh")
    .output()
    .expect("Snap Refresh Failed");

    println!("Command Output: {:#?}", String::from_utf8(dnf_release.stdout));
    println!("Status: {:#?}", dnf_release.status);
    println!("Error (If Error): {:#?}", String::from_utf8(dnf_release.stderr));
}
