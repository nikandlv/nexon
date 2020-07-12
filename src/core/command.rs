pub trait Command  {
    fn handle(&self, ctx: crate::Context);
    fn help(&self) -> &str;
    fn description(&self) -> &str;
    fn id(&self) -> String;
}
