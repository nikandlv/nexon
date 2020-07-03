pub trait Command {
    fn handle(&self,args: std::env::Args);
    fn help(&self);
    fn id() -> String;
}