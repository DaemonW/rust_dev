use actix_web::{
    get, http::header::ContentType, web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
};
use cookie::time::OffsetDateTime;
use std::ops::Add;
use std::time;

use crate::models::response::ServiceResponse;
use crate::StatusCode;

#[get("/")]
pub async fn hello() -> impl Responder {
    ServiceResponse::ok(serde_json::Value::Null)
        .customize()
        .with_status(StatusCode::TEMPORARY_REDIRECT)
        .insert_header(("Location", "index.html"))
}

#[get("/index.html")]
pub async fn index() -> impl Responder {
    let cookie = cookie::CookieBuilder::new("cookie", "sessionId=0000000-6445ab545d454f455c54")
        .secure(false)
        .http_only(true)
        .domain("localhost")
        .expires(OffsetDateTime::now_utc().saturating_add(cookie::time::Duration::seconds(300)))
        .finish();
    HttpResponse::Ok()
        .body("Welcome to actix web server!")
        .customize()
        .insert_header(("Set-Cookie", cookie.to_string()))
        .insert_header(("Location", "index.html"))
}

#[get("/echo/{msg}")]
pub async fn say(msg: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(msg.into_inner())
}
