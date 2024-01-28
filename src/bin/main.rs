use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use ztplib::{configuration::ZtpConfiguration, startup::ZtpServer, telemetry::ZtpTelemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = ZtpTelemetry::get_subscriber("ztp", "info", std::io::stdout);
    ZtpTelemetry::init_subscriber(subscriber);

    let config = ZtpConfiguration::get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
    let listener = TcpListener::bind((config.application.host, config.application.port))?;

    ZtpServer::run(listener, connection_pool)?.await
}
