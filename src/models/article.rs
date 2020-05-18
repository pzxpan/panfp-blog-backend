use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "article")]
pub struct Article {
    pub article_id: Option<i32>,
    pub user_id: Option<i32>,
    // pub path: Option<String>,
    pub view_count: Option<i32>,
    pub title: Option<String>,
    pub comment_count: Option<i32>,
    pub like_count: Option<i32>,
    pub date: Option<SystemTime>,
    // pub content_html: Option<String>,
    pub intro: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub article_id: Option<i32>,
    pub user_id: Option<i32>,
    pub comment_id: Option<i32>,
    pub category_id: Option<i32>,
    pub image_id: Option<i32>
}

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "article")]
pub struct ArticleDetail {
    pub article_id: Option<i32>,
    pub user_id: Option<i32>,
    pub view_count: Option<i32>,
    pub title: Option<String>,
    pub intro: Option<String>,
    pub comment_count: Option<i32>,
    pub like_count: Option<i32>,
    pub date: Option<SystemTime>,
    pub content_html: Option<String>,
    // pub category_id: Option<i32>
}

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "article")]
pub struct UserArticleDetail {
    pub article_id: Option<i32>,
    pub user_id: Option<i32>,
    pub view_count: Option<i32>,
    pub title: Option<String>,
    pub intro: Option<String>,
    pub comment_count: Option<i32>,
    pub like_count: Option<i32>,
    pub date: Option<SystemTime>,
    pub content_html: Option<String>,
    pub category_id: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewArticle {
    pub article_id: Option<i32>,
    pub user_id: i32,
    pub title: String,
    pub intro: String,
    pub category_id: i32,
    pub content_html: String,
    pub labels: Vec<i32>,
}

