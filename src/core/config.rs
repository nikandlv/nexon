
use crate::HashMap;

pub struct Config {
    pub pool: HashMap<String,HashMap<String,String>>
}

impl Config {
    pub fn initialize() -> Config {

let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config/alias")).unwrap()
        .merge(config::Environment::with_prefix("NEXON")).unwrap();
        return  Config{
            pool: settings.try_into::<HashMap<String,HashMap<String,String>>>().unwrap()
        }
    }
}
