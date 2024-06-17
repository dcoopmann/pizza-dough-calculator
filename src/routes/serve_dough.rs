use crate::pizza_dough::PizzaDough;
use actix_web::{web, Responder, Result};

pub async fn serve_dough() -> Result<impl Responder> {
    let dough = PizzaDough::new(2.0, "L".to_string(), "y".to_string());
    Ok(web::Json(dough))
}
