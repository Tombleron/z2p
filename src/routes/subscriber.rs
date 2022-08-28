/// Module for subscribe route
// TODO: change filename
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
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
#[tracing::instrument(
    name = "Adding new user",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
pub async fn subscribe(form: web::Form<Subscriber>, pool: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(&form, &pool).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[tracing::instrument(
    name = "Inserting new user to database",
    skip(data, pool),
)]
async fn insert_subscriber(
    data: &Subscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        data.email,
        data.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
