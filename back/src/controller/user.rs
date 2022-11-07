use crate::controller::lib::jwt_extractor::AccessTokenDecoded;
use crate::lib::my_error::MyError;
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
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
}

#[get("/user")]
async fn index(
    pool: Data<PgPool>,
    access_token_decoded: AccessTokenDecoded,
) -> MyResult<Json<UserDto>> {
    let user = user_query::find_by_id(&**pool, access_token_decoded.into().uid).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(MyError::new_unauthorized()),
    }
}

#[derive(Debug, Deserialize)]
struct Create {
    name: String,
    password: String,
}

#[post("/user")]
async fn create(pool: Data<PgPool>, form: Json<Create>) -> MyResult<Json<Tokens>> {
    let tokens = create_user(form.name.clone(), form.password.clone()).await?;
    let auth = get_user(tokens.access_token.clone()).await?;
    let user = User::create(auth.uid, form.name.clone())?;
    user.store(&**pool).await?;
    Ok(Json(tokens))
}
