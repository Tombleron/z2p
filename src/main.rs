use actix_web;

use sqlx::PgPool;
use z2p::{configuration::get_configuration, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Error loading configuration.");

    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Error connecting to db.");

    run(&config, connection).await
}
