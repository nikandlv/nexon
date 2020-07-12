

pub struct Config {
    pub pool: config::Config
}

impl Config {
    pub fn initialize() -> Config {
        let config_prefix = "config";
        let mut settings : config::Config = config::Config::default();
            settings
                .merge(config::File::with_name(format!("{}/{}", config_prefix, "nexon").as_str())).unwrap()
                .merge(config::Environment::with_prefix("NEXON")).unwrap();
        let configs = settings.get_array("nexon.configs").unwrap();
        for conf in configs {
            settings.merge(config::File::with_name(format!("{}/{}", config_prefix, conf).as_str())).unwrap();
        }
        Config{
            pool: settings
        }
    }
}
