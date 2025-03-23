mod arguments;
use arguments::argument_tokenizer::tokenize_arguments;

mod upgrades;
mod utility;

// Main Entry Point
fn main() -> () {
    tokenize_arguments();
    return ();
}
