mod arguments;
use arguments::prompt_command::command_prompt;

mod upgrades;
mod utility;

// Main Entry Point
fn main() -> () {
    command_prompt();
    return ();
}
