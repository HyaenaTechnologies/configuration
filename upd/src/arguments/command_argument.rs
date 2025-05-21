#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{process::ExitCode, string::String};

// Command Argument Definition
pub struct CommandArgument {
    pub name: String,
    pub description: String,
    pub event: ExitCode,
}
