pub use config::ConfigError;
use serde::Deserialize;
use slog::{o, Drain, Logger};
use slog_async;
use slog_envlogger;
use slog_term;
use actix_web::web::{ServiceConfig,post,route,resource,scope};

use crate::handlers::*;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn new() -> Self {
        let mut pg = deadpool_postgres::Config::default();
        pg.user = Some("test".to_string());
        pg.password = Some("111111".to_string());
        pg.dbname = Some("blog".to_string());
        Self {
            server: ServerConfig { host: "127.0.0.1".to_string(), port: 8080 },
            pg,
        }
    }
    // pub fn from_env() -> Result<Self, ConfigError> {
    //     let mut cfg = config::Config::new();
    //     cfg.
    //         cfg.merge(config::Environment::new())?;
    //     cfg.try_into()
    // }

    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }

    pub fn config_services(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/api")

                .service(
                    resource("/add_like")
                        .route(post().to(add_like)))
                .service(
                    resource("/is_like")
                        .route(post().to(is_like)))
                .service(
                    resource("/add_view_cnt")
                        .route(post().to(add_view_cnt)))

                .service(
                    resource("/cancel_like")
                        .route(post().to(cancel_like)))
                .service(
                    resource("/articles")
                        .route(post().to(articles)))
                .service(
                    resource("/hot_articles")
                        .route(post().to(hot_articles)))
                .service(
                    resource("/user_articles")
                        .route(post().to(user_articles)))
                .service(
                    resource("/article_detail")
                        .route(post().to(article_detail)))
                .service(
                    resource("/article_comments")
                        .route(post().to(article_comment_list)))
                .service(
                    resource("/user_article_comments")
                        .route(post().to(user_article_comment_list)))
                .service(
                    resource("/article_labels")
                        .route(post().to(article_labels)))
                .service(
                    resource("/all_labels")
                        .route(post().to(all_labels)))
                .service(
                    resource("/detail_user")
                        .route(post().to(detail_user)))
                .service(
                    resource("/categories")
                        .route(post().to(categories)))
                .service(
                    resource("/all_header_categories")
                        .route(post().to(all_header_categories)))
                .service(
                    resource("/recommend_categories")
                        .route(post().to(recommend_categories)))

                .service(
                    resource("/add_article_comment")
                        .route(post().to(add_article_comment)))
                .service(
                    resource("/add_article")
                        .route(post().to(add_article)))
                .service(
                    resource("/del_article_comment")
                        .route(post().to(delete_article_comment)))
                .service(
                    resource("/register")
                        .route(post().to(user_register)))
                .service(
                    resource("/login")
                        .route(post().to(user_login)))
                .service(
                    resource("/logout")
                        .route(post().to(logout)))
        );
    }
}
