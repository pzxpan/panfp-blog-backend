use actix_web::Responder;
use actix_multipart::{Field, Multipart};
use crate::models::errors::AppError;
use crate::models::result_response::ResultResponse;
use std::borrow::{BorrowMut, Borrow};
use actix_web::web::{block, Data, Json};
use std::io::Write;
use futures::{StreamExt, TryStreamExt};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use chrono::prelude::*;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crate::handlers::*;
use crate::db::*;

use deadpool_postgres::{Client, Pool, PoolError};
use slog::{crit, error, o, Logger};
use crate::models::image::Image;
use crate::models::errors::AppErrorCode::DbNotFoundErr;
use crate::models::app_state::AppState;
use crate::models::article::Id;

const FILE_SERVER_DIR: &str = "/root/blog/img/";
// const FILE_SERVER_DIR: &str = "/Users/panzhenxing/Desktop/panfp/panfp-blog/img/";

pub fn md5<S: Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}

fn get_file_path() -> String {
    let today = Local::today().format("%Y-%m-%d");
    let path = format!("{}", today);
    std::fs::create_dir_all(FILE_SERVER_DIR.to_string() + &path).unwrap();
    path
}

fn get_file_full_path(file_name: String) -> String {
    let path = get_file_path();
    let now = Local::now().to_rfc2822();
    path + "/" + &md5(file_name + &now)
}

pub async fn upload_file(mut payload: Multipart, state: Data<AppState>) -> Result<impl Responder, Error> {
    let mut user_id = 0;
    let mut source_name = "".to_string();
    let mut path = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();
        if name != "file" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                /* convert bytes to string and print it  (just for testing) */
                if let Ok(s) = std::str::from_utf8(&data) {
                    let data_string = s.to_string();
                    /* all not file fields of your form (feel free to fix this mess) */
                    match name {
                        "user_id" => user_id = data_string.parse::<i32>().unwrap(),
                        _ => println!("invalid field found"),
                    };
                };
            }
        } else {
            let file_name = content_type.get_filename().unwrap();
            source_name = file_name.to_string();
            let file_path = get_file_full_path(file_name.to_string());
            path = file_path.to_string();
            let local_path = FILE_SERVER_DIR.to_string() +  file_path.as_str();
            // File::create is blocking operation, use threadpool
            let mut f = block(|| std::fs::File::create(local_path))
                .await
                .unwrap();
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = block(move || f.write_all(&data).map(|_| f)).await?;
            }
        }
    }
    if user_id > 0 && path.len() > 0 && source_name.len() > 0 {
        let sublog = state.log.new(o!("handler" => "add_image"));
        let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
        let result = add_image(&client, user_id, path, source_name)
            .await?;
        if result.id > 0 {
            Ok(ResultResponse::normal(result))
        } else {
            Ok(ResultResponse::err(3000, "文件上传失败".to_string()))
        }
    } else {
        Ok(ResultResponse::normal("error"))
    }
}

pub async fn delete_img(state: Data<AppState>, id: Json<Id>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "delete_img"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    delete_image(&client, id.user_id.unwrap_or(0), id.image_id.unwrap_or(0))
        .await
        .map(|labels| ResultResponse::normal(labels))
        .map_err(log_error(sublog))
}

pub async fn get_user_img(state: Data<AppState>, id: Json<Id>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "get_user_image"));
    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;
    get_user_image(&client, id.user_id.unwrap_or(0))
        .await
        .map(|images| ResultResponse::normal(images))
        .map_err(log_error(sublog))
}