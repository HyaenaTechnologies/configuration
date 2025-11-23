mod arguments;
use crate::arguments::parser::parse;

mod git;

// Main Entry Point
fn main() -> () {
    parse();

    return ();
}
