use std::net::TcpListener;

use ztplib::{configuration::get_configuration, startup::ZtpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    ZtpServer::run(listener)?.await
}
