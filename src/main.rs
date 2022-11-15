extern crate core;

use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{cookie, web, App, HttpServer};
use clap::Parser;
use lettre::message::Mailbox;
use rpassword;
use std::error::Error;
use tracing::{error, info};
use tracing_subscriber;

use middleware::{SMTP_AUTH, SMTP_POSTER, SMTP_SERVER};
use models::db;

use crate::middleware::{err_detector, ALERT_EMAIL};
use crate::utils::email::check_smtp;

mod crypto;
mod handlers;
mod middleware;
mod models;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///listen address
    #[arg(short, long, default_value = "localhost")]
    addr: String,
    ///listen port
    #[arg(short, long, default_value = "8080")]
    port: u16,
    ///tls flag, default is off
    #[arg(short, long, default_value = "false")]
    tls: bool,
    ///email address of poster, if not set, will ignore internal server error alert
    #[arg(long, default_value = "")]
    smtp_poster: String,
    #[arg(long, default_value = "")]
    ///email password of poster, if not set, will read from terminal
    smtp_auth: String,
    ///SMTP server address
    #[arg(long, default_value = "")]
    smtp_server: String,
    ///email address to receive alert, if not set, will use the poster's email address
    #[arg(long, default_value = "")]
    alert: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let config: Args = Args::parse();
    check_config(&config).unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(-1);
    });
    let db_pool = db::init_conn_pool();
    let protocol = if config.tls { "https" } else { "http" };
    info!(
        "starting {} server at {}:{}",
        protocol, &config.addr, &config.port
    );
    let cookie_key = get_cookie_key();
    let server = HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::default())
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, err_detector))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                cookie_key.clone(),
            ))
            .app_data(web::Data::new(db_pool.clone()))
            .service(handlers::test::hello)
            .service(handlers::user::list_user)
            .service(handlers::user::create_user)
            .service(handlers::user::delete_user)
            .service(handlers::user::update_user_info)
            .service(handlers::test::index)
            .service(handlers::test::say);
        app
    });

    server.bind((config.addr, config.port))?.run().await
}

fn get_cookie_key() -> cookie::Key {
    let MAIN_KEY: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2,
    ];
    cookie::Key::derive_from(&MAIN_KEY)
}

fn check_config(args: &Args) -> Result<(), Box<dyn Error>> {
    let mut auth: String = String::default();
    if !args.smtp_poster.is_empty() {
        args.smtp_poster
            .parse::<Mailbox>()
            .map_err(|e| "illegal poster email format")?;
        if args.smtp_server.is_empty() {
            return Err("smtp server address is not set".into());
        }
        if args.smtp_auth.is_empty() {
            auth = rpassword::prompt_password("Input your smtp poster's password: ")?;
        } else {
            auth = args.smtp_auth.clone();
        }
        if let Err(e) = check_smtp(&args.smtp_poster, &auth, &args.smtp_server) {
            return Err("check authorization of poster email failed".into());
        }
    }
    if !args.alert.is_empty() {
        args.alert
            .parse::<Mailbox>()
            .map_err(|e| "illegal alert email format")?;
    }
    SMTP_AUTH.set(auth)?;
    SMTP_POSTER.set(args.smtp_poster.clone())?;
    SMTP_SERVER.set(args.smtp_server.clone())?;
    ALERT_EMAIL.set(args.alert.clone())?;
    Ok(())
}
