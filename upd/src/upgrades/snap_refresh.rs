use std::{
    io::{Error, StdoutLock, Write, stdout},
    process::{Command, ExitCode, Output},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Snap Refresh
pub fn refresh_snap() -> ExitCode {
    let snap_refresh: Result<Output, Error> = Command::new("snap").arg("refresh").output();
    let mut standard_output: StdoutLock = stdout().lock();

    match snap_refresh {
        Ok(refresh) => {
            standard_output.write_all(&refresh.stdout).unwrap();
            println!("Status: {}", refresh.status);
        }
        Err(error) => {
            eprintln!("Error Executing Snap Refresh: {}", error);
            return ExitCode::FAILURE;
        }
    };

    return ExitCode::SUCCESS;
}
