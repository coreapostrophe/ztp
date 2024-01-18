use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use crate::routes::{health_check::health_check, subscriptions::subscribe};

pub enum BindOption<'a> {
    SocketAddr((&'a str, u16)),
    Listener(TcpListener),
}

impl<'a> Into<BindOption<'a>> for (&'a str, u16) {
    fn into(self) -> BindOption<'a> {
        BindOption::SocketAddr(self)
    }
}

impl<'a> Into<BindOption<'a>> for TcpListener {
    fn into(self) -> BindOption<'a> {
        BindOption::Listener(self)
    }
}

pub struct ZtpServer;

impl ZtpServer {
    pub fn run<'a>(option: impl Into<BindOption<'a>>) -> std::io::Result<Server> {
        let http_server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        });

        let server = match option.into() {
            BindOption::SocketAddr(address) => http_server.bind(address)?.run(),
            BindOption::Listener(listener) => http_server.listen(listener)?.run(),
        };

        Ok(server)
    }
}
