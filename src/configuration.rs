use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub app_port: u16,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.yaml"))
        .build()?;

    settings.try_deserialize::<Configuration>()
}
