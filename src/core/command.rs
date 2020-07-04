pub trait Command {
    fn handle(&self, ctx: crate::context::Context);
    fn help(&self);
    fn id() -> String;
}