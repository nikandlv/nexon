use crate::core::{Command};
use crate::Context;

pub struct Handler {
    
}

impl Command for Handler {
    
    fn id(&self) -> String { String::from("docker") }
    fn handle(&self,_: Context) { 
        println!("Nexon beta");
        println!("Commands:");
        println!("  -h help");
     }
fn help(&self) -> &str { "Nexon docker command\nconfigure docker.toml and run the image commands as followed\nnexon docker image-name start|stop" }
fn description(&self) -> &str { "a docker utility for ease of use" }
}