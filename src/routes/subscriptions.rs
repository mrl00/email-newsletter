use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().finish();
    }

    match insert_subscriber(&form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn is_valid_name(name: &str) -> bool {
    let is_empty_or_whitespace = name.trim().is_empty();

    let is_too_long = name.graphemes(true).count() > 128;

    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

    let forbidden_characters_in_name = name.chars().any(|c| forbidden_characters.contains(&c));

    !(is_empty_or_whitespace || is_too_long || forbidden_characters_in_name)
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(pool, form)
)]
pub async fn insert_subscriber(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        insert into subscriptions (pk_subscription, tx_email, tx_name)
        values ($1, $2, $3)
        "#,
        Uuid::now_v7(),
        form.email,
        form.name,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
