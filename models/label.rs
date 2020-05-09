use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "label")]
pub struct Label {
    pub label_id: i32,
    pub name: String,
    pub label_alias: String,
    pub description: String
}