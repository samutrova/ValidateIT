mod handlers;
mod models;
mod services;

use std::{env, io};

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, middleware, web};

use crate::handlers::{fiscal_code, iban, vat_number};

pub struct Config {
    pub version: String,
    pub host: String,
    pub port: u16,
    pub workers: usize
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            version: env::var("VERSION").unwrap_or_else(|_| "1.0".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string())
                .parse().expect("Error parsing PORT number"),
            workers: env::var("WORKERS").unwrap_or_else(|_| "2".to_string())
                .parse().expect("Error parsing WORKERS number")
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Config::from_env();

    log::info!(
        "Starating server ( version {} ) on {}:{} with {} workers",
        config.version, config.host, config.port, config.workers
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::CONTENT_TYPE,
                http::header::AUTHORIZATION, 
                http::header::ACCEPT
            ])
            .allow_any_origin()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/fiscal-code")
                    .service(fiscal_code::validate_fiscal_code)
            )
            .service(
                web::scope("/vat-number")
                    .service(vat_number::validate_vat_number)
            )
            .service(
                web::scope("/iban")
                    .service(iban::validate_iban)
            )
    })
    .bind((config.host, config.port))?
    .workers(config.workers)
    .run()
    .await
}
