extern crate core;

use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{web, App, HttpServer};
use clap::Parser;
use rpassword;
use std::error::Error;
use tracing::info;
use tracing_subscriber;

use middleware::{SMTP_AUTH, SMTP_POSTER, SMTP_SERVER};
use models::db;

use crate::middleware::{err_detector, ALERT_EMAIL};
use crate::utils::email::check_smtp;

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
    ///email address of poster
    #[arg(long, default_value = "")]
    smtp_poster: String,
    ///email password of poster
    #[arg(long, default_value = "")]
    smtp_auth: String,
    ///SMTP server address
    #[arg(long, default_value = "")]
    smtp_server: String,
    ///email address of devops
    #[arg(long, default_value = "")]
    alert: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Args = Args::parse();
    check_config(&config).unwrap();
    tracing_subscriber::fmt::init();
    let db_pool = db::init_conn_pool();
    let protocol = if config.tls { "https" } else { "http" };
    info!(
        "starting {} server at {}:{}",
        protocol, &config.addr, &config.port
    );
    let server = HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::default())
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, err_detector))
            .app_data(web::Data::new(db_pool.clone()))
            .service(handlers::test::hello)
            .service(handlers::user::list_user)
            .service(handlers::user::create_user)
            .service(handlers::user::delete_user)
            .service(handlers::user::update_user_info)
            .service(handlers::test::say);
        app
    });

    server.bind((config.addr, config.port))?.run().await
}

fn check_config(args: &Args) -> Result<(), Box<dyn Error>> {
    if !args.smtp_poster.is_empty() && args.smtp_auth.is_empty() {
        let auth = rpassword::prompt_password("Input your smtp poster's password: ").unwrap();
        if let Err(e) = check_smtp(&args.smtp_poster, &auth, &args.smtp_server) {
            println!("{}", e);
            std::process::exit(-1);
        }
        SMTP_AUTH.set(auth.clone()).unwrap();
    }
    SMTP_POSTER.set(args.smtp_poster.clone()).unwrap();
    SMTP_SERVER.set(args.smtp_server.clone()).unwrap();
    ALERT_EMAIL.set(args.alert.clone()).unwrap();
    Ok(())
}
