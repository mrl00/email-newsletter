use std::net::TcpListener;

use email_newsletter::{
    configuration::{self, get_configuration, DatabaseSettings},
    startup,
};
use rnglib::RNG;
use sqlx::{Connection, Executor, PgConnection, PgPool};

pub struct TestApp {
    pub db_name: String,
    pub address: String,
    pub db_poll: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let address = format!("http:127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");

    let rng = RNG::try_from(&rnglib::Language::Demonic).unwrap();
    let db_name = format!("db_test_{}", rng.generate_name());
    configuration.database.database_name = db_name.clone();

    //let connection_pool = PgPool::connect(&configuration.database.connection_string())
    let connection_pool = configure_database(&configuration.database).await;

    let server = startup::run(listener, connection_pool.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        db_name,
        address,
        db_poll: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_without_db_string())
        .await
        .expect("Failed to connect to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    connection_pool
        .execute("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";")
        .await
        .expect("Failed to create uuid extension.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT sk_subscription, tx_email, tx_name FROM subscriptions",)
        .fetch_one(&app.db_poll)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.tx_email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.tx_name, "le guin");

    let _ = sqlx::query!(
        "DELETE FROM subscriptions WHERE sk_subscription=$1",
        saved.sk_subscription
    )
    .execute(&app.db_poll)
    .await
    .expect("Failed to delete data");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both"),
    ];

    for (invalid_doby, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_doby)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
