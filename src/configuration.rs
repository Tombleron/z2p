/// Module for configuration loading and processing
use config::{Config, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub application: ApplicationSettings,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    // Function for testing purposes
    // Connects to db without specified database
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
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
