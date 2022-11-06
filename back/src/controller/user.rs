use crate::lib::{
    auth::{create_user, get_user, Tokens},
    my_error::MyResult,
};
use crate::model::user::User;

use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;
use sqlx::MySqlPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Deserialize)]
struct Create {
    name: String,
    password: String,
}

#[post("/user")]
async fn create(pool: Data<MySqlPool>, form: Json<Create>) -> MyResult<Json<Tokens>> {
    let tokens = create_user(form.name.clone(), form.password.clone()).await?;
    let auth = get_user(tokens.access_token.clone()).await?;
    let user = User::create(auth.uid, form.password.clone())?;
    user.store(&**pool).await?;
    Ok(Json(tokens))
}
