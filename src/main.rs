use std::env::args;

pub mod core;
pub mod commands;

fn main() {
    let (script, args) = parse_arguments();
    let available_commands = commands::get_commands(); 
    println!("{}", script)
}

fn parse_arguments() -> (String,std::env::Args) {
    (args().nth(1).expect("provide a command to run"),args())
}
