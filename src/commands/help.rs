use crate::core::{Command};

pub struct Handler {
    
}

impl Command for Handler {
    fn handle(args: std::env::Args) { todo!() }
    fn id() -> &'static str { "-h" }
}