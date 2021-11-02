use crate::logs::{CreateLog, Logs};
use crate::error_handler::CustomError;
use actix_web::{HttpResponse, post, web};
use serde_json::json;


#[post("/logs")]
async fn create(log: web::Json<CreateLog>) -> Result<HttpResponse, CustomError> {
    let log = Logs::create(log.into_inner())?;
    Ok(HttpResponse::NoContent().into())
}



pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
}