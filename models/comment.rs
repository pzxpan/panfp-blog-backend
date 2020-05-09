use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use std::time::SystemTime;
use tokio_postgres::row::Row;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "comment")]
pub struct Comment {
    pub comment_id: i32,
    pub user_id: i32,
    pub article_id: i32,
    pub content: String,
    pub date: SystemTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayCommentInfo {
    pub comment_id: i32,
    pub user_id: i32,
    pub article_id: i32,
    pub content: String,
    pub date: SystemTime,
    pub nick_name:String,
    pub avatar:String
}

impl From<&Row> for DisplayCommentInfo {
    fn from(row: &Row) -> Self {
        Self {
            comment_id: row.try_get("comment_id").unwrap_or(0),
            user_id: row.try_get("user_id").unwrap_or(0),
            article_id: row.try_get("article_id").unwrap_or(0),
            content: row.try_get("content").unwrap_or("".to_string()),
            date: row.try_get("date").unwrap_or(SystemTime::now()),
            nick_name: row.try_get("nick_name").unwrap_or("".to_string()),
            avatar: row.try_get("avatar").unwrap_or("".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewComment {
    pub user_id: i32,
    pub article_id: i32,
    pub content: String
}

