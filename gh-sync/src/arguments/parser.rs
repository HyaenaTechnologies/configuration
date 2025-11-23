use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
    string::String,
    vec::Vec,
};

use super::{print_help::print_help_message, print_version::print_version_number};

use crate::github::synchronize_repositories::synch_repo;

// Command Line Argument Parser
pub fn parse() -> ExitCode {
    let command_line_arguments: Vec<String> = args().collect();
    let mut standard_output: StdoutLock = stdout().lock();

    if command_line_arguments.len() != 2 {
        writeln!(
            standard_output,
            "\x1b[31;1;3;4mCommand or Flag Required but not Both:\x1b[0m {:#?}",
            command_line_arguments
        )
        .unwrap();
        writeln!(standard_output, "").unwrap();
        print_help_message();
        eprintln!("\x1b[31;1;3;4mError(1) - Exiting GitHub Syncrhonize\x1b[0m");
        return ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "help" | "--h" => {
                print_help_message();
            }
            "version" | "--v" => {
                print_version_number();
            }
            &_ => {
                synch_repo(command_line_arguments[1].trim());
            }
        };
    };

    return ExitCode::SUCCESS;
}
