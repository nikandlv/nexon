use std::env::args;

pub mod core;
pub mod commands;
pub mod context;

use crate::core::Command;
pub use context::Context;

fn main() {
    let (script, arguments) = parse_arguments();
    let config = core::Config::initialize();
    let console = core::Console::initialize();
    let context = context::Context{
        config,
        arguments,
        console
    };
    let available_commands = commands::get_commands(); 
    match available_commands.get(&script) {
        Some(cmd) => {&cmd.handle(context);},
        _ => context.console.abort(format!("Command {} not found", script)),
    }
}

fn parse_arguments() -> (String,std::env::Args) {
    (args().nth(1).expect("provide a command to run"),args())
}
