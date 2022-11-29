use crate::controller::lib::*;
use crate::lib::my_error::*;
use crate::model::{command::*, query::*};

use actix_web::{
    get, patch,
    web::{Data, Json, Path, ServiceConfig},
};
use chrono::NaiveDate;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(show);
    cfg.service(update);
}

#[get("/user/daily_records/{recorded_on}")]
async fn show(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    recorded_on: Path<NaiveDate>,
) -> MyResult<Json<Option<DailyRecordDto>>> {
    let daily_record =
        find_daily_record(&**pool, at.into_inner().uid, recorded_on.into_inner()).await?;
    Ok(Json(daily_record))
}

#[patch("/user/daily_records/{recorded_on}")]
async fn update(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    recorded_on: Path<NaiveDate>,
    form: Json<UpdateDailyRecord>,
) -> MyResult<Json<()>> {
    let user_id = at.into_inner().uid;
    let recorded_on = recorded_on.into_inner();
    let form = form.into_inner();

    let mut daily_record =
        DailyRecord::one_of_user_by_recorded_on(&**pool, user_id, recorded_on).await?;
    daily_record.update(form);
    daily_record.upsert(&**pool).await?;

    Ok(Json(()))
}
