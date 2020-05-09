use crate::models::result_response::ResultResponse;
use crate::constants::constant::STR_EMPTY;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;
use deadpool_postgres::PoolError;
use tokio_postgres::error::Error;

#[derive(Debug)]
pub enum AppErrorCode {
    Normal,
    AuthErr,
    DbNotFoundErr,
    ParaNotFoundErr,
    FileNotFoundErr,
}

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub err_code: AppErrorCode,
}

impl From<PoolError> for AppError {
    fn from(error: PoolError) -> AppError {
        AppError {
            message: STR_EMPTY.to_string(),
            err_code: AppErrorCode::DbNotFoundErr,
        }
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            message: STR_EMPTY.to_string(),
            err_code: AppErrorCode::DbNotFoundErr,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
    fn error_response(&self) -> HttpResponse {
        let code = match self.err_code {
            AppErrorCode::AuthErr => 1000,
            AppErrorCode::DbNotFoundErr => 2000,
            AppErrorCode::ParaNotFoundErr => 3000,
            AppErrorCode::FileNotFoundErr => 4000,
            _ => 10000
        };
        ResultResponse::err(code, self.message.clone())
    }
}
