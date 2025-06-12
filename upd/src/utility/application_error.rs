#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::primitive::str;

// Application Error Definition
pub struct ApplicationError<T> {
    pub error_message: &'static str,
    pub error_value: T,
}
