use sqlx::PgPool;
use z2p::{configuration::get_configuration, logging::*, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup global setting for the logger
    let subscriber = get_subscriber("z2p".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Load app configuration (NOTE: stored in config.yaml)
    let config = get_configuration().expect("Error loading configuration.");

    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Error connecting to db.");

    run(&config, connection).await
}
