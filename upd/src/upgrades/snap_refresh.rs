use std::{
    io::Error,
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
};

// Snap Refresh
pub fn refresh_snap() -> ExitCode {
    let snap_refresh: Result<Output, Error> = Command::new("snap").arg("refresh").output();

    match snap_refresh {
        Ok(refresh) => {
            println!("Command Output: {:#?}", String::from_utf8(refresh.stdout));
            println!("Status: {}", refresh.status);
        }
        Err(error) => {
            eprintln!("Error Refreshing Snaps {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
