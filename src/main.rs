use std::collections::HashMap;
use std::env::args;

pub mod core;
pub mod commands;
pub mod context;
pub use colored::*;

use crate::core::Command;
pub use context::Context;

fn main() {
    let console = core::Console::initialize();
    let (script_opt,mut arguments) = parse_arguments();
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
        arguments: &mut arguments,
        console
    };

    // context.console.write(format!("{:?}", context.config.pool));

    // let alias_list = context.config.pool.get_table("alias");
    // if alias_list.is_err() {
    //     return context.console.abort(format!("Invalid alias configuration, {}", alias_list.err().unwrap()))
    // }
    // for alias in alias_list.unwrap() {
    //     println!("{:?}",alias)
    // }
    
    let available_commands = commands::get_commands(); 
    match available_commands.get(&script) {
        Some(cmd) => {&cmd.handle(context);},
        _ => match script.as_str() {
            "help" => {help(&mut context, available_commands);},
            _ => {context.console.abort(format!("Command {} not found", script))} 
        },
    }
}

fn help(ctx: &mut Context, cmds: HashMap<String,Box<dyn Command>> ){
    if ctx.arguments.len() > 2 {
        let arg = &ctx.arguments[2];
        match cmds.get(arg) {
            Some(cmd) => {
                ctx.console.write(format!("{}", &cmd.help()));
                ctx.console.exit(0)
            },
            _ =>  {ctx.console.abort(format!("Command {} not found", &arg))} 
        }
    }

    ctx.console.write(format!("{}","Available Commands:".blue()));
    for arg in cmds {
        ctx.console.write(format!(" {}: {}",arg.0.yellow(), arg.1.description()))
    }
}

fn parse_arguments() -> (std::option::Option<std::string::String>, Vec<String>) {
    let mut arg_list  = vec!();
    for arg in args() {
        arg_list.append(&mut vec!(arg))
    }
    (args().nth(1),arg_list)
}
