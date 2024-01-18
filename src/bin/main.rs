use std::net::TcpListener;

use sqlx::PgPool;
use ztplib::{configuration::get_configuration, startup::ZtpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("to connect to Postgres.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    ZtpServer::run(listener, connection_pool)?.await
}
