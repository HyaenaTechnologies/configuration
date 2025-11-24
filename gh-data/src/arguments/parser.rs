use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
    string::String,
    vec::Vec,
};

use super::{print_help::print_help_message, print_version::print_version_number};

use crate::github::{
    write_folliwng::following, write_repositories::repositories, write_stars::starred,
};

// Command Line Argument Tokenizer
pub fn tokenize_arguments() -> ExitCode {
    let command_line_arguments: Vec<String> = args().collect();
    let mut standard_output: StdoutLock = stdout().lock();

    if command_line_arguments.len() != 3 {
        writeln!(
            standard_output,
            "\x1b[31;1;3;4mFlag and User Required:\x1b[0m {:#?}",
            command_line_arguments
        )
        .unwrap();
        writeln!(standard_output, "").unwrap();
        print_help_message();
        eprintln!("\x1b[31;1;3;4mError(1) - Exiting GitHub Data Tool\x1b[0m");
        return ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "--following" | "--f" => {
                following(command_line_arguments[2].trim());
            }
            "help" | "--h" => {
                print_help_message();
            }
            "--repositories" | "--r" => {
                repositories(command_line_arguments[2].trim());
            }
            "--starred" | "--s" => {
                starred(command_line_arguments[2].trim());
            }

            "version" | "--v" => {
                print_version_number();
            }
            &_ => {
                writeln!(
                    standard_output,
                    "\x1b[31;1;3;4mUknown Command or Flag:\x1b[0m {:#?}",
                    command_line_arguments[1].trim()
                )
                .unwrap();
                writeln!(standard_output, "").unwrap();
                print_help_message();
                eprintln!("\x1b[31;1;3;4mError(1) - Exiting GitHub Data Tool\x1b[0m");
                return ExitCode::FAILURE;
            }
        };
    };

    return ExitCode::SUCCESS;
}
