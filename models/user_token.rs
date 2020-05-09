use crate::models::user::User;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use jsonwebtoken::{
    EncodingKey,
    Header,
};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_MONTH: i64 = 60 * 60 * 24 * 7* 30; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub login_session: String,
}

impl Default for UserToken {
    fn default() -> UserToken {
        UserToken {
            iat: 0,
            exp: 0,
            user: "".to_string(),
            login_session: "".to_string(),
        }
    }
}

impl UserToken {
    pub fn generate_token(login: &User) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_MONTH,
            user: login.email.clone(),
            login_session: login.login_session.clone(),
        };
        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }
}