use crate::core::{Command};
use crate::Context;

pub struct Handler {
    
}

impl Command for Handler {
    
    fn id(&self) -> String { String::from("-h") }
    fn handle(&self,_: Context) { 
        println!("Nexon beta");
        println!("Commands:");
        println!("  -h help");
     }
fn help(&self) -> &str { "a long help" }
fn description(&self) -> &str { "a command which is written for test" }
}