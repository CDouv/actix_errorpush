use crate::logs::{CreateLog, Logs};
use crate::error_handler::CustomError;
use actix_web::{HttpResponse, post, web};
use serde_json::json;

// #[get("/logs")]

// async fn find_all() -> Result<HttpResponse, CustomError> {
//     let logs = Logs::find_all()?;
//     Ok(HttpResponse::Ok().json(logs))
// }

// #[get("/logs/{id}")]

// async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
//     let log = Logs::find(id.into_inner())?;
//     Ok(HttpResponse::Ok().json(log))
// }

#[post("/logs")]
async fn create(log: web::Json<CreateLog>) -> Result<HttpResponse, CustomError> {
    let log = Logs::create(log.into_inner())?;
    Ok(HttpResponse::NoContent().into())
}


// #[put("/logs/{id}")]
// async fn update(
//     id: web::Path<i32>,
//     log: web::Json<CreateLog>,
// ) -> Result<HttpResponse, CustomError> {
//     let log = Logs::update(id.into_inner(), log.into_inner())?;
//     Ok(HttpResponse::Ok().json(log))
// }
// #[delete("/logs/{id}")]
// async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
//     let deleted_log = Logs::delete(id.into_inner())?;
//     Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_log })))

pub fn init_routes(config: &mut web::ServiceConfig) {
    // config.service(find_all);
    // config.service(find);
    config.service(create);
    // config.service(update);
    // config.service(delete);
}