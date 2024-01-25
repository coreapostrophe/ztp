use std::net::TcpListener;

use sqlx::PgPool;
use ztplib::{configuration::ZtpConfiguration, startup::ZtpServer, telemetry::ZtpTelemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = ZtpTelemetry::get_subscriber("ztp", "info");
    ZtpTelemetry::init_subscriber(subscriber);

    let config = ZtpConfiguration::get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("to connect to Postgres.");
    let listener = TcpListener::bind(("127.0.0.1", 8080))?;

    ZtpServer::run(listener, connection_pool)?.await
}
