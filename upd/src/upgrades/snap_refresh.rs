use std::{
    process::{Command, ExitCode, Output},
    string::String,
};

// Snap Refresh
pub fn refresh_snap() -> ExitCode {
    let dnf_release: Output = Command::new("snap")
        .arg("refresh")
        .output()
        .expect("Snap Refresh Failed");

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
