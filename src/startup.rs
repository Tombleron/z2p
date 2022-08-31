/// Startup configuration module
use crate::configuration::Configuration;

use super::routes::*;

use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

/// This needed to easily start app while testing
/// Combines all routes for the app
pub fn routing(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check))
        .route("/subscribe", web::post().to(subscribe));
}

pub async fn run(cfg: &Configuration, connection: PgPool) -> std::io::Result<()> {
    let connection = web::Data::new(connection);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(routing)
            .app_data(connection.clone())
    })
    .bind((cfg.application.host.to_owned(), cfg.application.port))?
    .run()
    .await
}
