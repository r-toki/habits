use crate::lib::{
    auth::{create_user, get_user, Tokens},
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
    let tokens = create_user(form.name.clone(), form.password.clone()).await?;
    let auth = get_user(tokens.access_token.clone()).await?;
    println!("{:?}", auth);
    Ok(Json(tokens))
}
