use crate::core::{Command};

pub struct Handler {
    
}

impl Command for Handler {
    
    fn id() -> String { String::from("-h") }
    fn handle(&self,args: std::env::Args) { 
        println!("Nexon beta");
        println!("Commands:");
        println!("  -h help");
     }
fn help(&self) { todo!() }
}