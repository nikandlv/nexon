use crate::core::{Config, Console};

pub struct Context { 
    pub config: Config,
    pub arguments: std::env::Args,
    pub console: Console
}