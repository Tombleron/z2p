/// Module for configuration loading and processing
use config::{Config, Environment, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{postgres::PgConnectOptions, ConnectOptions};

#[derive(Deserialize)]
pub struct Configuration {
    pub application: ApplicationSettings,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let mut opt = self.without_db().database(&self.database_name);
        opt.log_statements(tracing::log::LevelFilter::Trace);
        opt
    }
}

/// Get app configuration from config files
pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let config_dir = base_path.join("cfg");

    let environment: AppEnvironment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Error parsing APP_ENV");

    let s = Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(File::from(config_dir.join("base")))
        // Add in the current environment file
        .add_source(File::from(config_dir.join(environment.as_str())))
        .add_source(Environment::with_prefix("app").separator("__"))
        .build()?;

    s.try_deserialize()
}

/// Possible environments for app
enum AppEnvironment {
    Local,
    Production,
}

impl AppEnvironment {
    fn as_str(&self) -> &'static str {
        match self {
            AppEnvironment::Local => "local",
            AppEnvironment::Production => "prod",
        }
    }
}

impl TryFrom<String> for AppEnvironment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(AppEnvironment::Local),
            "prod" => Ok(AppEnvironment::Production),
            other => Err(format!(
                "{other} is not supported environment. Use either `local` or `prod`."
            )),
        }
    }
}
