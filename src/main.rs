use std::collections::HashMap;
use std::env::args;

pub mod core;
pub mod commands;
pub mod context;

use crate::core::Command;
pub use context::Context;

fn main() {
    let console = core::Console::initialize();
    let (script_opt,arguments) = parse_arguments();
    if script_opt.is_none()  {
        console.logo();
        console.error(format!("No command provided"));
        console.help(format!("run `nexon help`"));
        console.exit(1);
    }
    let script = script_opt.unwrap();
    let config = core::Config::initialize();

    let mut context = context::Context{
        config,
        arguments,
        console
    };


    println!("{:?}",context.config.pool);
    
    let available_commands = commands::get_commands(); 
    match available_commands.get(&script) {
        Some(cmd) => {&cmd.handle(context);},
        _ => match script.as_str() {
            "help" => {help(&mut context, available_commands);},
            _ => {context.console.abort(format!("Command {} not found", script))} 
        },
    }
}

fn help(ctx: &mut Context, cmds: HashMap<String,impl Command> ){
    let argument = ctx.arguments.nth(2);
    if argument.is_some() {
        let arg = argument.unwrap();
        match cmds.get(&arg) {
            Some(cmd) => {
                ctx.console.write(format!("{}", &cmd.help()));
                ctx.console.exit(0)
            },
            _ =>  {ctx.console.abort(format!("Command {} not found", &arg))} 
        }
    }

    ctx.console.write(format!("Available Commands:"));
    for arg in cmds {
        ctx.console.write(format!(" {}: {}",arg.0, arg.1.description()))
    }
}

fn parse_arguments() -> (std::option::Option<std::string::String>, std::env::Args) {
    (args().nth(1),args())
}
