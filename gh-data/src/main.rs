mod arguments;
use arguments::argument_tokenizer::tokenize_arguments;

mod github;

// Main Entry Point
fn main() -> () {
    tokenize_arguments();

    return ();
}
