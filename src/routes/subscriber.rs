/// Module for subscribe route
// TODO: change filename
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Subscriber {
    name: String,
    email: String,
}

/// Inserts new subscriber into db
///
/// If data is invalid or there's some error with database
/// this method will yield Internal Server Error
pub async fn subscribe(
    data: web::Form<Subscriber>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %data.email,
        subscriber_name= %data.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        data.email,
        data.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError()
        }
    }
}
