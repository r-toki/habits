use super::lib::AccessTokenDecoded;
use crate::lib::Error;
use crate::service::daily_records::*;
use actix_web::{
    get, patch,
    web::{Data, Json, Path, ServiceConfig},
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(show);
    cfg.service(update);
}

#[get("/daily_records/{start_of_recorded_at}")]
async fn show(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<DateTime<Utc>>,
) -> Result<Json<Vec<DailyRecord>>, Error> {
    todo!();
}

#[patch("/daily_records/{start_of_recorded_at}")]
async fn update(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<DateTime<Utc>>,
    form: Json<UpdateHabitDailyRecord>,
) -> Result<Json<()>, Error> {
    todo!();
}
