use actix_web::web::{HttpResponse};
use actix_http::Response;
use std::string::ToString;
use serde::{Serialize};

#[derive(Serialize)]
pub struct ResultResponse {}

impl ResultResponse {
    pub fn err(code:i32,msg:String) -> Response {
        HttpResponse::Ok().header("ok","ok").json(
        ApiResBody {
            code,
            msg,
            data:{}
        })
    }
    pub fn normal(data: impl Serialize) -> Response {
        HttpResponse::Ok().header("ok","ok").json(
            ApiResBody {
                code: 0,
                msg: "".to_string(),
                data
        })
    }
}

#[derive(Serialize)]
pub struct ApiResBody<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: T
}


