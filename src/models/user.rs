use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "user")]
pub struct User {
    pub user_id: i32,
    pub password: String,
    pub email: String,
    pub register_time: Option<SystemTime>,
    pub nick_name: Option<String>,
    pub profession: Option<String>,
    pub level: Option<i32>,
    pub avatar: Option<String>,
    pub login_session: String,
    pub expire: Option<SystemTime>,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub nick_name: Option<String>,
    pub level: Option<i32>,
    pub profession: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDisplayInfo {
    pub user_id: i32,
    pub email: String,
    pub nick_name: Option<String>,
    pub profession: Option<String>,
    pub level: Option<i32>,
    pub avatar: Option<String>,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDetailInfo {
    pub email: String,
    pub user_id: Option<i32>,
    pub nick_name: Option <String>,
    pub profession: Option<String>,
    pub level: Option<i32>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangePassword {
    pub user_id: i32,
    pub password: String,
    pub new_password: String
}

impl UserDetailInfo {
    pub fn new(user: User) -> UserDetailInfo {
        UserDetailInfo {
            user_id: Some(user.user_id),
            email: user.email,
            nick_name: user.nick_name,
            profession: user.profession,
            level: user.level,
            avatar: user.avatar,
        }
    }
}

impl UserDisplayInfo {
    pub fn new(user: User, token: String) -> UserDisplayInfo {
        UserDisplayInfo {
            token,
            user_id: user.user_id,
            email: user.email,
            nick_name: user.nick_name,
            profession: user.profession,
            level: user.level,
            avatar: user.avatar,
        }
    }
}

