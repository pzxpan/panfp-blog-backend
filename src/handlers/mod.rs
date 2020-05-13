pub mod article_handler;
pub mod user_handler;
use crate::models::{user::{User, NewUser, UserDetailInfo},
                    article::Article,
                    article::Id,
                    label::Label,
                    comment::Comment,
                    app_state::AppState,
                    result_response::ResultResponse};
use crate::models::user_token::UserToken;
use crate::models::user::UserDisplayInfo;
use crate::models::errors::AppError;
use crate::db;
use actix_web::web::{Json, Data};
use actix_web::Responder;
use deadpool_postgres::{Client, Pool, PoolError};
use slog::{crit, error, o, Logger};
use crate::models::comment::NewComment;
use crate::models::errors::AppErrorCode::ParaNotFoundErr;
use crate::models::category::CategoryArray;
use crate::models::article::NewArticle;

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err: PoolError| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error creating client");
        AppError::from(err)
    })
}
fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let log = log.new(o!(
            "cause" => err.message.clone()
        ));
        error!(log, "{}", err.message);
        err
    }
}




