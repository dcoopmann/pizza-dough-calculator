use crate::pizza_dough::PizzaDough;
use actix_web::{web, Responder, Result};
use tracing::info;

#[tracing::instrument]
pub async fn serve_dough(dough_dto: web::Json<PizzaDough>) -> Result<impl Responder> {
    let dough = PizzaDough::new(
        dough_dto.portions,
        dough_dto.size.clone(),
        dough_dto.yeast_type.clone(),
    );
    info!("Generated Pizza dough DTO: {:?}", dough);
    Ok(web::Json(dough))
}
