use crate::core::config::Config;

pub struct Context {
    pub config: Config,
    pub arguments: std::env::Args
}