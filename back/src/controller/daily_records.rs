use super::lib::AccessTokenDecoded;
use crate::lib::Error;
use crate::service::daily_records::*;
use actix_web::{
    get, patch,
    web::{Data, Json, Path, Query, ServiceConfig},
};
use chrono::NaiveDate;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(show);
    cfg.service(update);
}

#[get("/daily_records/{recorded_on}")]
async fn show(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<NaiveDate>,
    query: Query<GetDailyRecord>,
) -> Result<Json<Option<DailyRecord>>, Error> {
    get_daily_record(
        &**pool,
        at.into_inner().uid,
        path.into_inner(),
        query.into_inner(),
    )
    .await
    .map(Json)
}

#[patch("/daily_records/{recorded_on}")]
async fn update(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<NaiveDate>,
    form: Json<UpdateDailyRecord>,
) -> Result<Json<()>, Error> {
    update_daily_record(
        &**pool,
        at.into_inner().uid,
        path.into_inner(),
        form.into_inner(),
    )
    .await
    .map(Json)
}
