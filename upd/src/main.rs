mod arguments;
use arguments::argument_tokenizer::tokenize_arguments;

mod hypertext_markup;
mod hypertext_transfer;
mod service;
mod upgrades;
mod utility;

// Main Entry Point
fn main() -> () {
    tokenize_arguments();

    return ();
}
