#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::primitive::u8;

// Process Option Definition
pub type ProcessOption = u8;

// Process Option Type
pub const VALUE_OPTION: ProcessOption = 0;
pub const NULL_OPTION: ProcessOption = 1;
