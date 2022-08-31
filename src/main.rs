use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use z2p::{configuration::get_configuration, logging::*, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup global setting for the logger
    let subscriber = get_subscriber("z2p".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Load app configuration (NOTE: stored in config.yaml)
    let config = get_configuration().expect("Error loading configuration.");

    let connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(config.database.connection_string().expose_secret())
        .expect("Error connecting to db.");

    run(&config, connection).await
}
