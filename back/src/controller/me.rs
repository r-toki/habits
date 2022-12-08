use super::lib::AccessTokenDecoded;
use crate::lib::Error;
use crate::service::me::*;
use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
};
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
}

#[get("/me")]
async fn index(pool: Data<PgPool>, at: AccessTokenDecoded) -> Result<Json<Me>, Error> {
    todo!();
}

#[post("/me")]
async fn create(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    form: Json<CreateMe>,
) -> Result<Json<()>, Error> {
    todo!();
}
