use crate::controller::lib::jwt_extractor::AccessTokenDecoded;
use crate::lib::{
    auth::{create_user, get_user, Tokens},
    my_error::MyResult,
};
use crate::model::{
    user::User,
    user_query::{self, UserDto},
};

use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;
use sqlx::MySqlPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
}

#[get("/user")]
async fn index(
    pool: Data<MySqlPool>,
    access_token_decoded: AccessTokenDecoded,
) -> MyResult<Json<UserDto>> {
    user_query::find(&**pool, access_token_decoded.into().uid)
        .await
        .map(Json)
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
    let user = User::create(auth.uid, form.name.clone())?;
    user.store(&**pool).await?;
    Ok(Json(tokens))
}
