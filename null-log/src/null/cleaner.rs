use std::{
    fs::{DirEntry, ReadDir, read_dir},
    io::{Error, StdoutLock, Write, stdout},
    path::PathBuf,
    primitive::{str, u8},
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Clean Logs by Copying the contents of /dev/null to the Log Files
pub fn clean_logs(directory: &str) -> u8 {
    let log_directory: ReadDir = read_dir(directory).unwrap();
    let mut standard_output: StdoutLock = stdout().lock();

    for directory_entry in log_directory {
        let entry: DirEntry = directory_entry.unwrap();
        let path: PathBuf = entry.path();

        if path.is_file() {
            let clean_logs: Result<Output, Error> =
                Command::new("cp").arg("/dev/null").arg(path).output();

            match clean_logs {
                Ok(cleaning) => {
                    writeln!(standard_output, "\x1b[32;1;3mCleaning Log File...\x1b[0m").unwrap();
                    standard_output.write_all(&cleaning.stdout).unwrap();
                    writeln!(standard_output, "{}", cleaning.status).unwrap();
                }
                Err(error) => {
                    eprintln!("\x1b[31;1;3;4mError Cleaning Log Files:\x1b[0m {}", error);
                    exit(1);
                }
            };
        } else {
            writeln!(standard_output, "Skipping...").unwrap();
        }
    }

    return 0;
}
