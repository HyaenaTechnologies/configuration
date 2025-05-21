#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{process::ExitCode, string::String};

// Flag Argument Definition
pub struct FLagArgument {
    pub name: String,
    pub description: String,
    pub event: ExitCode,
}
