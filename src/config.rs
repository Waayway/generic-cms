use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,

    pub app_url: Option<String>,
    pub app_port: Option<u16>,
}

static mut ENV: Option<Config> = None;

impl Config {
    pub fn get_env() -> Self {
        if let Some(cfg) = unsafe { ENV.clone() } {
            return cfg;
        }

        if cfg!(debug_assertions) && dotenv::dotenv().is_err() {
            panic!("Unable to find .env Needed for development")
        };

        match envy::from_env::<Config>() {
            Ok(config) => {
                unsafe {
                    ENV = Some(config.clone());
                }
                config
            }
            Err(error) => {
                panic!(
                    "Possibly missing value, see error below for more info: {}",
                    error
                )
            }
        }
    }
}
