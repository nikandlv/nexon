mod help;
mod docker;

use std::collections::HashMap;
use crate::core::Command;

pub fn get_commands() -> HashMap<String,Box<dyn Command>> {
    let mut commands = HashMap::<String,Box<dyn Command>>::new();  
      
    commands.insert(help::Handler{}.id(),Box::new(help::Handler{}));
    commands.insert(docker::Handler{}.id(),Box::new(docker::Handler{}));
    
    commands
}