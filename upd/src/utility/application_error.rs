#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::string::String;

// Application Error Definition
pub struct ApplicationError<T> {
    pub error_message: String,
    pub error_value: T,
}
