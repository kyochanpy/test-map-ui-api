use config::{Config, ConfigError, File};
use map_ui_adapter::clients::mysql::MySqlConfig;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct MySql {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

impl MySqlConfig for MySql {
    fn host(&self) -> String {
        self.host.clone()
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn user(&self) -> String {
        self.user.clone()
    }

    fn password(&self) -> String {
        self.password.clone()
    }

    fn database(&self) -> String {
        self.database.clone()
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    pub mysql: MySql,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("app_config/local.toml"))
            .build()?;
        config.try_deserialize()
    }
}
