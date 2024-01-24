use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::{health_check::health_check, subscriptions::subscribe};

pub enum BindOption<'a> {
    SocketAddr((&'a str, u16)),
    Listener(TcpListener),
}

impl<'a> From<(&'a str, u16)> for BindOption<'a> {
    fn from(value: (&'a str, u16)) -> Self {
        BindOption::SocketAddr(value)
    }
}

impl<'a> From<TcpListener> for BindOption<'a> {
    fn from(value: TcpListener) -> Self {
        BindOption::Listener(value)
    }
}

pub struct ZtpServer;

impl ZtpServer {
    pub fn run<'a>(option: impl Into<BindOption<'a>>, db_pool: PgPool) -> std::io::Result<Server> {
        let db_pool = web::Data::new(db_pool);

        let http_server = HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
                .app_data(db_pool.clone())
        });

        let server = match option.into() {
            BindOption::SocketAddr(address) => http_server.bind(address)?.run(),
            BindOption::Listener(listener) => http_server.listen(listener)?.run(),
        };

        Ok(server)
    }
}
