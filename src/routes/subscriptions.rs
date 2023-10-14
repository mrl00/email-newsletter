use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    sqlx::query!(
        r#"
        insert into subscriptions (tx_name, tx_email, dh_subscribed_at) values($1, $2, now());
        "#,
        form.name,
        form.email
    )
    .execute(pool.get_ref())
    .await
    .expect("Failed to execute query");

    HttpResponse::Ok().finish()
}
