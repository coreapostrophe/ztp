use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use ztplib::{configuration::get_configuration, startup::ZtpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("to connect to Postgres.");
    let listener = TcpListener::bind(("127.0.0.1", 8080))?;

    ZtpServer::run(listener, connection_pool)?.await
}
