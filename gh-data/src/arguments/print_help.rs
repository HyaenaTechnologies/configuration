use std::io::{StdoutLock, Write, stdout};

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(
        standard_output,
        "\t\x1b[32;1;3;4mGitHub Data Tool\x1b[0m\n
        \x1b[32;1;3mCommands:\tDescription:\x1b[0m\n           
        \x1b[32;3mhelp\x1b[0m\t\tPrint Commands and Flags        
        \x1b[32;3mversion\x1b[0m\t\tPrint Version Number        
        \x1b[32;1;3mFlags:\t\tDescription:\x1b[0m\n
        \x1b[32;3m--following\x1b[0m\t\tWrite Following Data
        \x1b[32;3m--f\x1b[0m\t\tWrite Following Data
        \x1b[32;3m--h\x1b[0m\t\tPrint Commands and Flags
        \x1b[32;3m--repositories\x1b[0m\t\tWrite Repository Data
        \x1b[32;3m--r\x1b[0m\t\tWrite Repository Data
        \x1b[32;3m--starred\x1b[0m\t\tWrite Starred Data
        \x1b[32;3m--s\x1b[0m\t\tWrite Starred Data        
        \x1b[32;3m--v\x1b[0m\t\tPrint Version Number        
        "
    )
    .unwrap();

    return ();
}
