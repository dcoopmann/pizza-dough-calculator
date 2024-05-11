use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn configure_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!(
        "Starting to serve Pizza via Http on: {}",
        listener.local_addr().unwrap()
    );

    let server =
        HttpServer::new(move || App::new().route("/health-check", web::get().to(health_check)))
            .listen(listener)?
            .run();
    Ok(server)
}
