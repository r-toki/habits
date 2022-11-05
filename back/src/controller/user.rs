use crate::lib::{
    auth::{createUser, Tokens},
    my_error::MyResult,
};

use actix_web::{
    post,
    web::{Json, ServiceConfig},
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Deserialize)]
struct Create {
    name: String,
    password: String,
}

#[post("/user")]
async fn create(form: Json<Create>) -> MyResult<Json<Tokens>> {
    createUser(form.name.clone(), form.password.clone())
        .await
        .map(|tokens| Json(tokens))
}
