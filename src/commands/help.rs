use crate::core::{Command};
use crate::context::Context;

pub struct Handler {
    
}

impl Command for Handler {
    
    fn id() -> String { String::from("-h") }
    fn handle(&self,ctx: Context) { 
        println!("Nexon beta");
        println!("Commands:");
        println!("  -h help");
     }
fn help(&self) { todo!() }
}