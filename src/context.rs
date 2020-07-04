use crate::core::Config;

pub struct Context {
    pub config: Config,
    pub arguments: std::env::Args
}