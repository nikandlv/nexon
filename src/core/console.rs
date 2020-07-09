use colored::*;

pub struct Console {

}

impl Console {

    pub fn logo(&self) {
        println!("{}","
███╗   ██╗███████╗██╗  ██╗ ██████╗ ███╗   ██╗
████╗  ██║██╔════╝╚██╗██╔╝██╔═══██╗████╗  ██║
██╔██╗ ██║█████╗   ╚███╔╝ ██║   ██║██╔██╗ ██║
██║╚██╗██║██╔══╝   ██╔██╗ ██║   ██║██║╚██╗██║
██║ ╚████║███████╗██╔╝ ██╗╚██████╔╝██║ ╚████║
╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝                       
        ".purple().bold())
    }

    pub fn write(&self ,text: String) {
        println!("{}",text)
    }
    pub fn abort(&self ,text: String) {
        println!("{}: {}","Error".red(),text);
        std::process::exit(1)
    }

    pub fn error(&self ,text: String) {
        println!("{}: {}","Error:".red(),text);
    }

    pub fn help(&self ,text: String) {
        println!("{}: {}","Tip:".cyan(),text);
    }

    pub fn exit(&self ,code : i32) {
        std::process::exit(code)
    }


    pub fn initialize() -> Console {
        return  Console{}
    }
}
