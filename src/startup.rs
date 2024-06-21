use crate::routes::health_check;
use crate::routes::serve_dough;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing::info;

pub fn configure_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    info!(
        "Starting to serve Pizza via Http on: {}",
        listener.local_addr().unwrap()
    );

    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/serve-dough", web::post().to(serve_dough))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
