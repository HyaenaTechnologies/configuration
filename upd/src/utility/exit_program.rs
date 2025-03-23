use std::process::exit;

// Successful Exit of the Program
pub fn successful_exit() -> ! {
    println!("Exiting Systsem Update Daemon");
    exit(0);
}

// Program Exited with Error
pub fn error_exit() -> ! {
    println!("Exiting - Error(1) - Systsem Update Daemon");
    exit(1);
}
