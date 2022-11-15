use crate::models::response::{ServiceError, ServiceResponse};
use actix_web::{get, http, web, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> Result<ServiceResponse, ServiceResponse> {
    // ServiceResponse::new(http::StatusCode::OK).with_content("Hello, World!")
    ServiceResponse::bad_request(ServiceError::ErrIllegalQueryParam, "Hello, World!")
}

#[get("/echo/{msg}")]
pub async fn say(msg: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(msg.into_inner())
}
