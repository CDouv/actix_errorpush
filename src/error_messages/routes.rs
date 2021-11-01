use crate::error_messages::{ValErrorMessage, ValErrorMessages};
use crate::error_handler::CustomError;
use actix_web::{delete,get,post,put,web,HttpResponse};
use serde_json::json;

#[get("/val_error_messages")]

async fn find_all() -> Result<HttpResponse, CustomError> {
    let val_error_messages = ValErrorMessages::find_all()?;
    Ok(HttpResponse::Ok().json(val_error_messages))
}

#[get("/val_error_messages/{id}")]

async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let val_error_message = ValErrorMessages::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(val_error_message))
}
#[post("/val_error_messages")]
async fn create(val_error_message: web::Json<ValErrorMessage>) -> Result<HttpResponse, CustomError> {
    let val_error_message = ValErrorMessages::create(val_error_message.into_inner())?;
    Ok(HttpResponse::Ok().json(val_error_message))
}
#[put("/val_error_messages/{id}")]
async fn update(
    id: web::Path<i32>,
    val_error_message: web::Json<ValErrorMessage>,
) -> Result<HttpResponse, CustomError> {
    let val_error_message = ValErrorMessages::update(id.into_inner(), val_error_message.into_inner())?;
    Ok(HttpResponse::Ok().json(val_error_message))
}
#[delete("/val_error_messages/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_error_message = ValErrorMessages::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_error_message })))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}