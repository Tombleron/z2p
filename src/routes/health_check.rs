/// Module for health check route
use actix_web::{HttpResponse, Responder};

/// Health check methods exists to check if app is still alive and responding
/// This method need just to send OK on any request
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
