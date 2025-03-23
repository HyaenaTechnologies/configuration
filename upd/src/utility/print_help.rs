// Print Help Command Output
pub fn print_help_message() -> () {
    println!("Systsem Update Daemon");
    println!("");
    println!("");
    println!("Commands:					Description:");
    println!("");
    println!("apt-upgrade               APT Upgrade");
    println!("dnf-release               DNF System Release Upgrade");
    println!("dnf-upgrade               DNF Upgrade");
    println!("help                      Print List of Commands and Flags");
    println!("snap-refresh              Snap Refresh");
    println!("ubuntu-release            Ubuntu System Releade Upgrade");
    println!("version                   Print Version Number");
    println!("");
    println!("Flags:				    Description:");
    println!("--au                      APT Upgrade");
    println!("--dr                      DNF System Release Upgrade");
    println!("--du                      DNF Upgrade");
    println!("--h                       Print List of Commands and Flags");
    println!("--sr                      Snap Refresh");
    println!("--ur                      Ubuntu System Releade Upgrade");
    println!("--v                       Print Version Number");
    println!("");

    return ();
}
