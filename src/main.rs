use std::env::args;

pub mod core;
pub mod commands;
pub mod context;

use crate::core::Command;

fn main() {
    let (script, args) = parse_arguments();
    let config = core::config::Config::initialize();
    let context = context::Context{
        config
    };
    let available_commands = commands::get_commands(); 
    match available_commands.get(&script) {
        Some(cmd) => {&cmd.handle(args);},
        _ => panic!("Command {} not found", script),
    }
}

fn parse_arguments() -> (String,std::env::Args) {
    (args().nth(1).expect("provide a command to run"),args())
}
