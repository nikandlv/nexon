
pub struct Console {

}

impl Console {
    pub fn write(&self ,text: String) {
        println!("{}",text)
    }
    pub fn abort(&self ,text: String) {
        println!("{}",text);
        std::process::exit(1)
    }
    

    pub fn initialize() -> Console {
        return  Console{}
    }
}
