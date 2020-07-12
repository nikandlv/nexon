use crate::core::{Command};
use crate::Context;

pub struct Handler {
    
}

impl Command for Handler {
    
    fn id(&self) -> String { String::from("docker") }
    fn handle(&self,ctx: Context) { 
        let total_args = ctx.arguments.len();
        ctx.console.write(format!("{:?}",total_args));
        match total_args {
            2 => {
                ctx.console.write(format!("interactive shell"))
            },
            3 => {
                match ctx.arguments[2].as_str() {
                    "start" => {
                        ctx.console.write(format!("start"));
                    },
                    "stop" => {
                        ctx.console.write(format!("stop"));
                    },
                    _ => {
                        ctx.console.error(format!("docker action not found"));
                        ctx.console.help(format!("use either start|stop"));
                        ctx.console.exit(1)
                    },
                }
            },
            _ => {
                ctx.console.write(format!("{}",self.help()))
            }
        }
        
     }
fn help(&self) -> &str { "Nexon docker command\nconfigure docker.toml and run the image commands as followed\nnexon docker start|stop image-name or\nnexon docker start" }
fn description(&self) -> &str { "a docker utility for ease of use" }
}