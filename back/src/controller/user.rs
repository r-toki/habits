use crate::controller::lib::*;
use crate::lib::my_error::*;
use crate::model::{query::*, table::*};

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
async fn index(pool: Data<PgPool>, at: AccessTokenDecoded) -> MyResult<Json<UserDto>> {
    let user = find_user(&**pool, at.into_inner().id).await?;
    Ok(Json(user))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Create {
    display_name: String,
}

#[post("/user")]
async fn create(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    form: Json<Create>,
) -> MyResult<Json<()>> {
    let user = TUser::create(at.into_inner().id, form.display_name.clone())?;
    user.upsert(&**pool).await?;
    Ok(Json(()))
}
