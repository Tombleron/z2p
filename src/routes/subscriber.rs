use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Subscriber {
    name: String,
    email: String,
}

// TODO: entire function, i guess?
pub async fn subscribe(data: web::Form<Subscriber>) -> impl Responder {
    HttpResponse::Ok()
}