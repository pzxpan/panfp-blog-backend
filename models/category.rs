use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "category")]
pub struct Category {
    pub category_id: i32,
    pub name: String,
    pub category_alias: String,
    pub description: String,
    pub parent_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryArray {
    pub category_id: i32,
    pub name: String,
    pub subcategory: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "hot_category")]
pub struct HotCategory {
    pub hot_id: i32,
    pub category_id: i32,
    pub name: String,
    pub category_alias: String,
    pub description: String,
    pub parent_id: i32,
}