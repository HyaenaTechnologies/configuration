use std::process::ExitCode;

mod arguments;
use arguments::argument_tokenizer::tokenize_arguments;

mod upgrades;
mod utility;

// Main Entry Point
fn main() -> ExitCode {
    tokenize_arguments();

    return ExitCode::SUCCESS;
}
