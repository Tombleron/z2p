use crate::configuration::Configuration;

use super::routes::*;

use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

pub fn routing(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check))
        .route("/subscribe", web::post().to(subscribe));
}

pub async fn run(cfg: &Configuration, connection: PgPool) -> std::io::Result<()> {
    let connection = web::Data::new(connection);

    HttpServer::new(move || App::new().configure(routing).app_data(connection.clone()))
        .bind(("127.0.0.1", cfg.app_port))?
        .run()
        .await
}
