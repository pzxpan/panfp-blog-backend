mod models;
mod config;
mod db;
mod handlers;
mod middleware;
mod utils;
mod constants;

use crate::models::app_state::AppState;
use crate::middleware::authen_middleware::Authentication;
use crate::config::Config;

use actix_web::{http, App, HttpServer};
use actix_cors::Cors;
use actix_service::Service;
use futures::future::FutureExt;
// use dotenv::dotenv;
use slog::info;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // dotenv().ok();
    // let config = Config::from_env().unwrap();
    let config = Config::new();
    let pool = config.pg.create_pool(NoTls).unwrap();
    let log = Config::configure_log();
    info!(
        log,
        "Starting server at http://{}:{}", config.server.host, config.server.port
    );
    let app_state = AppState {
        pool: pool.clone(),
        log: log.clone(),
    };
    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            // .wrap(middleware::Logger::default())
        // [("Access-Control-Expose-Headers", "*" ),
        //     ("Access-Control-Allow-Headers", "Content-Type,Access-Token,Authorization,x-requested-with"),
        //     ("Access-Control-Allow-Methods","*"),
        //     ("Access-Control-Allow-Credentials","true"),
        //     ("Access-Control-Allow-Origin","*")
        // ]
            .wrap(Cors::new() // allowed_origin return access-control-allow-origin: * by default
                .send_wildcard()
                .allowed_methods(vec!["OPTIONS","GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT,http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                                      http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,http::header::ACCESS_CONTROL_ALLOW_METHODS])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish())
            .wrap(Authentication)
            .wrap_fn(|req, srv| {
                srv.call(req).map(|res| res)
            })
            .configure(Config::config_services)
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}