use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "image")]
pub struct Image {
    pub id: i32,
    pub path: String,
    pub user_id: i32,
    pub source_name: String,
    pub create_time: SystemTime,
}