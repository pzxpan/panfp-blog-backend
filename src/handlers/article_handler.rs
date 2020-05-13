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
use crate::handlers::*;

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

pub async fn add_article(new_article: Json<NewArticle>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "add_article"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    db::add_aritcle(&client, &new_article)
        .await
        .map(|id| ResultResponse::normal(id))
        .map_err(log_error(sublog))
}

pub async fn update_article(new_article: Json<NewArticle>, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "update_article"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    db::update_article(&client, &new_article)
        .await
        .map(|id| ResultResponse::normal(id))
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