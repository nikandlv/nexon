pub trait Command {
    fn handle(&self, ctx: crate::Context);
    fn help(&self);
    fn id() -> String;
}