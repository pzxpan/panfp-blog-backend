use crate::{constants::constant::*, utils::token_utils};
use crate::config::Config;
use crate::models::app_state::AppState;
use crate::models::user_token::UserToken;
use crate::handlers;

use actix_service::{Service, Transform};
use actix_web::{
    http::Method,
    dev::{ServiceRequest, ServiceResponse},
    Error,
};
use futures::{future::{ok, Ready}, Future};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_postgres::NoTls;
use actix_web::http::header::{HeaderName, HeaderValue};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = true;
        let mut use_token = UserToken::default();

        let headers = req.headers_mut();

        headers.append(HeaderName::from_static("content-length"), HeaderValue::from_static("true"));
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in NEED_AUTH.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = false;
                }
            }
        }
        if !authenticate_pass {
            if let Some(authen_header) = req.headers_mut().get(AUTHORIZATION) {
                if let Ok(authen_str) = authen_header.to_str() {
                    if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                        let token = authen_str[6..authen_str.len()].trim();
                        if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                            use_token = token_data.claims;
                        }
                    }
                }
            }
        }
        let fut = self.service.call(req);
        if authenticate_pass {
            Box::pin(async {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async {
                //用client方式访问数据库，
                //从全局环境中拿到数据库的访问资源，如果从req里取，则会有数据竞争，无法通过编译;
                let config = Config::new();
                let pool = config.pg.create_pool(NoTls).unwrap();
                let log = Config::configure_log();
                let app_state = AppState {
                    pool: pool.clone(),
                    log: log.clone(),
                };

                //如果鉴权失败,会直接返回Error数据给客户端
                handlers::verify_token(use_token, app_state).await?;

                //如果鉴权成功,继续poll
                let res = fut.await?;
                Ok(res)
            })
        }
    }
}
