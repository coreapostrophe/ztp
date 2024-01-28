use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use ztplib::{
    configuration::{DatabaseSettings, ZtpConfiguration},
    startup::ZtpServer,
    telemetry::ZtpTelemetry,
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info";
    let subscriber_name = "test";

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            ZtpTelemetry::get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        ZtpTelemetry::init_subscriber(subscriber);
    } else {
        let subscriber =
            ZtpTelemetry::get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        ZtpTelemetry::init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    pub async fn spawn() -> Self {
        Lazy::force(&TRACING);

        let domain = "127.0.0.1";
        let listener =
            TcpListener::bind(&format!("{}:0", domain)).expect("Failed to bind to random port.");
        let port = listener.local_addr().unwrap().port();
        let address = format!("{}:{}", domain, port);

        let mut config =
            ZtpConfiguration::get_configuration().expect("Failed to read configuration.");
        config.database.database_name = Uuid::new_v4().to_string();

        let connection_pool = Self::configure_database(&config.database).await;

        let server =
            ZtpServer::run(listener, connection_pool.clone()).expect("Failed to bind address");

        let _ = tokio::spawn(server);

        Self {
            address,
            db_pool: connection_pool,
        }
    }

    async fn configure_database(config: &DatabaseSettings) -> PgPool {
        let mut connection = PgConnection::connect_with(&config.without_db())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
            .await
            .expect("Failed to create database.");

        let connection_pool = PgPool::connect_with(config.with_db())
            .await
            .expect("Failed to connect to Postgres.");
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");

        connection_pool
    }
}

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("http://{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = TestApp::spawn().await;

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("http://{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
