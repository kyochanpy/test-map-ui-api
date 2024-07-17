use std::sync::Arc;

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySql, Pool,
};

#[derive(Clone, Debug)]
pub struct MySqlConnection {
    pub pool: Arc<Pool<MySql>>,
    pub database: String,
}

pub trait MySqlConfig {
    fn host(&self) -> String;
    fn port(&self) -> u16;
    fn user(&self) -> String;
    fn password(&self) -> String;
    fn database(&self) -> String;
}

impl MySqlConnection {
    pub async fn new(config: &impl MySqlConfig) -> Self {
        let connect_options = MySqlConnectOptions::new()
            .host(&config.host())
            .port(config.port())
            .username(&config.user())
            .password(&config.password())
            .database(&config.database());
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect_with(connect_options)
            .await
            .inspect_err(|err| eprintln!("Failed to connect to MySQL: {}", err))
            .unwrap();
        Self {
            pool: Arc::new(pool),
            database: config.database(),
        }
    }
}
