use actix_web::{HttpResponse, Responder};
use tracing::info;

#[tracing::instrument]
pub async fn health_check() -> impl Responder {
    info!("Health Check called");
    HttpResponse::Ok().finish()
}
