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

pub async fn articles(article_type: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "articles"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::get_articles(&client, article_type.category_id.unwrap_or(0))
        .await
        .map(|articles| ResultResponse::normal(articles))
        .map_err(log_error(sublog))
}

pub async fn hot_articles(article_type: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "articles"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::get_hot_articles(&client, article_type.category_id.unwrap_or(0))
        .await
        .map(|articles| ResultResponse::normal(articles))
        .map_err(log_error(sublog))
}

pub async fn user_articles(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "user_articles"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::get_user_articles(&client, id.user_id.unwrap_or(0))
        .await
        .map(|articles| ResultResponse::normal(articles))
        .map_err(log_error(sublog))
}

pub async fn article_detail(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "article_detail"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    let result = db::article_detail(&client, id.article_id.unwrap_or(0)).await;
    match result {
        Ok(detail) => {
            Ok(ResultResponse::normal(detail))
        }
        Err(e) => Err(e)
    }
}

pub async fn add_view_cnt(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "article_detail"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    let result = db::add_view_cnt(&client, id.article_id.unwrap_or(0)).await;
    Ok(ResultResponse::normal(0))
}

pub async fn article_comment_list(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "article_comment_list"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::article_comment_list(&client, id.article_id.unwrap_or(0))
        .await
        .map(|comments| ResultResponse::normal(comments))
        .map_err(log_error(sublog))
}

pub async fn user_article_comment_list(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "user_article_comment_list"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::user_article_comment_list(&client, id.article_id.unwrap_or(0))
        .await
        .map(|comments| ResultResponse::normal(comments))
        .map_err(log_error(sublog))
}

pub async fn article_labels(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "article_labels"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::article_labels(&client, id.article_id.unwrap_or(0))
        .await
        .map(|labels| ResultResponse::normal(labels))
        .map_err(log_error(sublog))
}

pub async fn all_labels(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "all_labels"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::all_labels(&client)
        .await
        .map(|labels| ResultResponse::normal(labels))
        .map_err(log_error(sublog))
}

pub async fn add_article_comment(new_comment: Json<NewComment>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "add_article_comment"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::add_aritcle_comment(&client, new_comment)
        .await
        .map(|comment_id| ResultResponse::normal(comment_id))
        .map_err(log_error(sublog))
}

pub async fn add_article(new_article: Json<NewArticle>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "add_article"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    let labels = &new_article.labels;
    db::add_aritcle(&client, &new_article)
        .await
        .map(|id| ResultResponse::normal(id))
        .map_err(log_error(sublog))
}

pub async fn add_like(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "add_like"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::add_like(&client, id.user_id.unwrap_or(0), id.article_id.unwrap_or(0))
        .await
        .map(|labels| ResultResponse::normal(labels))
        .map_err(log_error(sublog))
}

pub async fn cancel_like(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "cancel_like"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::cancel_like(&client, id.user_id.unwrap_or(0), id.article_id.unwrap_or(0))
        .await
        .map(|labels| ResultResponse::normal(labels))
        .map_err(log_error(sublog))
}

pub async fn is_like(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "is_like"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::is_like(&client, id.user_id.unwrap_or(0), id.article_id.unwrap_or(0))
        .await
        .map(|labels| ResultResponse::normal(labels))
        .map_err(log_error(sublog))
}

pub async fn delete_article_comment(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "delete_article_comment"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::del_aritcle_comment(&client, id.user_id.unwrap_or(0), id.article_id.unwrap_or(0), id.comment_id.unwrap_or(0))
        .await
        .map(|labels| ResultResponse::normal(labels))
        .map_err(log_error(sublog))
}


pub async fn categories(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "categories"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::categories(&client, id.category_id.unwrap_or(0))
        .await
        .map(|cate| ResultResponse::normal(cate))
        .map_err(log_error(sublog))
}

pub async fn all_header_categories(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "article_comment_list"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    let arraysub = ["".to_string(), "文章".to_string(), "教程".to_string(), "漫谈".to_string()].to_vec();
    let mut array = Vec::new();
    for mut i in 1..4 {
        let result = db::categories(&client, i)
            .await?;
        if result.len() > 0 {
            array.push(CategoryArray { name: String::from(arraysub.get(i as usize).unwrap()), category_id: i, subcategory: result });
        }
    }
    Ok(ResultResponse::normal(array))
}

pub async fn recommend_categories(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "recommend_categories"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::hot_categories(&client)
        .await
        .map(|cate| ResultResponse::normal(cate))
        .map_err(log_error(sublog))
}


pub async fn user_register(user: Json<NewUser>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "register"
    ));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::register(&client, &user)
        .await
        .map(|item| ResultResponse::normal(item))
        .map_err(log_error(sublog))
}

pub async fn user_login(user: Json<NewUser>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "login"
    ));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    let result = db::login(&client, &user).await;
    match result {
        Ok(mut tmp_user) => {
            let login_session_str = db::generate_login_session();
            let result = db::update_login_session_to_db(&client, &user.email, &login_session_str).await;

            match result {
                Ok(_) => {
                    tmp_user.login_session = login_session_str;
                    let token = UserToken::generate_token(&tmp_user);
                    let client_user = UserDisplayInfo::new(tmp_user, token);
                    Ok(ResultResponse::normal(client_user))
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn logout(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "article_comment_list"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::logout(&client, id.user_id.unwrap_or(0))
        .await
        .map(|is_logout| ResultResponse::normal(is_logout))
        .map_err(log_error(sublog))
}

pub async fn detail_user(id: Json<Id>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "detail_user"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::detail_user(&client, id.user_id.unwrap_or(0))
        .await
        .map(|user| ResultResponse::normal(UserDetailInfo::new(user)))
        .map_err(log_error(sublog))
}

pub async fn verify_token(token_data: UserToken, state: AppState) -> Result<i32, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "verify_token"
    ));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::verify_token(&client, token_data.user, token_data.login_session).await
}


