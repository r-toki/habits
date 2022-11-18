use crate::controller::lib::jwt_extractor::AccessTokenDecoded;
use crate::lib::my_error::MyError;
use crate::lib::my_error::MyResult;
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
    let auth_user = access_token_decoded.into();
    let user = user_query::find_by_id(&**pool, auth_user.uid).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(MyError::new_unauthorized()),
    }
}

#[derive(Debug, Deserialize)]
struct Create {
    display_name: String,
}

#[post("/user")]
async fn create(
    pool: Data<PgPool>,
    access_token_decoded: AccessTokenDecoded,
    form: Json<Create>,
) -> MyResult<Json<()>> {
    let auth_user = access_token_decoded.into();
    let user = User::create(auth_user.uid, form.display_name.clone())?;
    user.store(&**pool).await?;
    Ok(Json(()))
}
