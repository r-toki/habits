use crate::controller::lib::jwt_extractor::AccessTokenDecoded;
use crate::lib::my_error::MyResult;
use crate::model::aggregate::{find_daily_record, DailyRecordDto};

use actix_web::{
    get,
    web::{Data, Json, Path, ServiceConfig},
};
use chrono::NaiveDate;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(show);
}

#[get("/user/daily_records/{recorded_on}")]
async fn show(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    recorded_on: Path<NaiveDate>,
) -> MyResult<Json<DailyRecordDto>> {
    let daily_record =
        find_daily_record(&**pool, at.into_inner().id, recorded_on.into_inner()).await?;
    Ok(Json(daily_record))
}
