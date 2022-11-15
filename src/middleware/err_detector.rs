use std::error::Error;
use std::ops::Sub;
use std::sync::Mutex;
use std::thread::spawn;

use actix_web::body::MessageBody;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, http};
use chrono::Local;
use lettre::transport::smtp::authentication::Credentials;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use tracing::{error, info, warn};

use crate::utils::email::send_mail;

const EMAIL_INTERVAL: i64 = 600;
static EMAIL_STAMP: Lazy<Mutex<chrono::DateTime<Local>>> =
    Lazy::new(|| Mutex::new(Local::now().sub(chrono::Duration::seconds(EMAIL_INTERVAL))));

pub static SMTP_POSTER: OnceCell<String> = OnceCell::new();
pub static SMTP_AUTH: OnceCell<String> = OnceCell::new();
pub static SMTP_SERVER: OnceCell<String> = OnceCell::new();
pub static ALERT_EMAIL: OnceCell<String> = OnceCell::new();
pub fn err_detector<B>(
    mut err: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, actix_web::Error>
where
    B: MessageBody,
{
    if let http::StatusCode::INTERNAL_SERVER_ERROR = err.status() {
        let stamp = EMAIL_STAMP.lock().unwrap();
        let diff = chrono::Local::now() - *stamp;
        if diff.lt(&chrono::Duration::seconds(EMAIL_INTERVAL)) {
            let t = (*stamp).format("%Y-%m-%d %H:%M:%S").to_string();
            info!("alert email is sent too frequently, last send at: {}", t);
            return Ok(ErrorHandlerResponse::Response(err.map_into_left_body()));
        }

        spawn(|| {
            if let Err(e) = report_err() {
                error!("send email failed: {}", e)
            } else {
                let mut stamp = EMAIL_STAMP.lock().unwrap();
                *stamp = chrono::Local::now();
            }
        });
    }
    err.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(err.map_into_left_body()))
}

fn report_err() -> Result<(), Box<dyn Error>> {
    let poster = SMTP_POSTER.get().unwrap();
    let auth = SMTP_AUTH.get().unwrap();
    let server = SMTP_SERVER.get().unwrap();
    if poster.is_empty() || auth.is_empty() || server.is_empty() {
        warn!("detect internal error, but email config is not set, skip sending alert email");
        return Ok(());
    }
    let c = Credentials::new(poster.into(), auth.into());
    let mut alert_email = ALERT_EMAIL.get().unwrap();
    if alert_email.is_empty() {
        alert_email = poster;
    }
    send_mail(
        alert_email,
        "internal server occurred!",
        "Ops!!! Internal server error is occurred. Please check you server log",
        poster,
        c,
        server,
    )
}
