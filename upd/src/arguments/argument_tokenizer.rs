use std::{env::args, process::ExitCode};

use crate::utility::{print_help::print_help_message, print_version::print_version_number};

use crate::upgrades::{
    apt_upgrade::upgrade_apt, dnf_release::release_dnf, dnf_upgrade::upgrade_dnf,
    snap_refresh::refresh_snap, ubuntu_release::release_ubuntu,
};

// Command Line Argument Tokenizer
pub fn tokenize_arguments() -> ExitCode {
    let command_line_arguments: Vec<String> = args().collect();

    if command_line_arguments.len() != 2 {
        println!(
            "Command or Flag Required but not Both: {:#?}",
            command_line_arguments
        );
        print_help_message();
        println!("Error(1) - Exiting System Update Daemon");
        return ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "apt-upgrade" | "--au" => {
                upgrade_apt();
            }
            "dnf-release" | "--dr" => {
                release_dnf();
            }
            "dnf-upgrade" | "--du" => {
                upgrade_dnf();
            }
            "help" | "--h" => {
                print_help_message();
            }
            "snap-refresh" | "--sr" => {
                refresh_snap();
            }
            "ubuntu-release" | "--ur" => {
                release_ubuntu();
            }
            "version" | "--v" => {
                print_version_number();
            }
            &_ => {
                println!(
                    "Uknown Command or Flag: {:#?}",
                    command_line_arguments[1].trim()
                );
                print_help_message();
                println!("Error(1) - Exiting System Update Daemon");
                return ExitCode::FAILURE;
            }
        };
    };

    return ExitCode::SUCCESS;
}
