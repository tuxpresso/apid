use crate::routes::{health_check, settings};

use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub fn run(listener: TcpListener, boilerd_port: u16) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/settings", web::post().to(settings))
            .app_data(web::Data::new(boilerd_port))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
