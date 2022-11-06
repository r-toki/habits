use crate::lib::auth::get_user;
use crate::lib::{
    auth::Auth,
    my_error::{MyError, MyResult},
};

use actix_web::{http::header, FromRequest};
use lazy_static::lazy_static;
use regex::Regex;
use std::future::Future;
use std::pin::Pin;

lazy_static! {
    static ref RE_BEARER: Regex = Regex::new(r"^Bearer\s(.*)$").unwrap();
}

pub struct AccessTokenDecoded(Auth);

impl AccessTokenDecoded {
    pub fn into(self) -> Auth {
        self.0
    }
}

impl FromRequest for AccessTokenDecoded {
    type Error = MyError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = extract_bearer_token(&req)?;
            let auth = get_user(token).await?;
            Ok(AccessTokenDecoded(auth))
        })
    }
}

fn extract_bearer_token(req: &actix_web::HttpRequest) -> MyResult<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|authorization| {
            RE_BEARER
                .captures(authorization)
                .and_then(|captures| captures.get(1))
        })
        .map(|v| v.as_str().to_owned())
        .ok_or_else(|| MyError::new_unauthorized())
}
