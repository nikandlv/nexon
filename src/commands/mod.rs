mod help;

use std::collections::HashMap;
use crate::core::Command;

pub fn get_commands() -> HashMap<String,impl Command> {
    let mut commands = HashMap::new();    
    commands.insert(help::Handler::id(),help::Handler{});
    commands
}