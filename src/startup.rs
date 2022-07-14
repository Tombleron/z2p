use crate::configuration::Configuration;

use super::routes::*;

use actix_web::{web, App, HttpServer};


pub fn routing(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check))
        .route("/subscribe", web::post().to(subscribe));
}

pub async fn run(cfg: &Configuration) -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routing))
        .bind(("127.0.0.1", cfg.app_port))?
        .run()
        .await
}