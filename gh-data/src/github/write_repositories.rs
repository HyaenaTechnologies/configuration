use std::{
    fs::File,
    io::{BufRead, Error, Lines, StdoutLock, Write, stdout},
    path::PathBuf,
    primitive::{str, u8},
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Write GitHub Repository Data to a Markdown File
pub fn repositories(user: &str) -> u8 {
    let github_repositories: Result<Output, Error> = Command::new("gh")
        .arg("api")
        .arg(user)
        .arg("--jq")
        .arg(".[].html_url")
        .arg("--paginate")
        .output();
    let file_path: PathBuf = PathBuf::from("./repositories.md");
    let markdown_file: Result<File, Error> = File::create(file_path);
    let mut standard_output: StdoutLock = stdout().lock();

    match github_repositories {
        Ok(listing) => {
            standard_output.write_all(&listing.stdout).unwrap();
            writeln!(standard_output, "{}", listing.status).unwrap();

            let repositories: Lines<&[u8]> = listing.stdout.lines();

            match markdown_file {
                Ok(mut file) => {
                    writeln!(file, "# Repositories").unwrap();
                    writeln!(file, "").unwrap();
                    for repository in repositories {
                        writeln!(file, "- {}", repository.unwrap()).unwrap();
                    }
                    writeln!(file, "").unwrap();
                }
                Err(error) => {
                    eprintln!("\x1b[31;1;3;4mError Creating File:\x1b[0m {}", error);
                    exit(1);
                }
            };
        }
        Err(error) => {
            eprintln!(
                "\x1b[31;1;3;4mError Listing GitHub Repositories:\x1b[0m {}",
                error
            );
            exit(1);
        }
    };

    return 0;
}
