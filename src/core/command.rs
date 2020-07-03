pub trait Command {
    fn handle(args: std::env::Args);
    fn id() -> &'static str;
}