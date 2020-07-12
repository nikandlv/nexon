use crate::core::{Config, Console};

pub struct Context<'a> { 
    pub config: Config,
    pub arguments: &'a mut Vec<String>,
    pub console: Console
}