use std::{
    io::{StdoutLock, Write, stdout},
    primitive::u8,
};

// Print Help Command Output
pub fn print_help_message() -> u8 {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(
        standard_output,
        "\t\x1b[32;1;3;4mGitHub Synchronize\x1b[0m\n
        \x1b[32;1;3mCommands:\tDescription:\x1b[0m\n
        \x1b[32;3musers/GitHubUser/repos\x1b[0m\t\tInput GitHub User   
        \x1b[32;3mhelp\x1b[0m\t\tPrint Commands and Flags        
        \x1b[32;3mversion\x1b[0m\t\tPrint Version Number        
        \x1b[32;1;3mFlags:\t\tDescription:\x1b[0m\n        
        \x1b[32;3m--h\x1b[0m\t\tPrint Commands and Flags        
        \x1b[32;3m--v\x1b[0m\t\tPrint Version Number
        "
    )
    .unwrap();

    return 0;
}
