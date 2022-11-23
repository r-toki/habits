use crate::controller::lib::*;
use crate::lib::my_error::*;
use crate::model::{command::*, query::*, table::*};

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
) -> MyResult<Json<DailyRecordDto>> {
    let daily_record =
        find_daily_record(&**pool, at.into_inner().id, recorded_on.into_inner()).await?;
    Ok(Json(daily_record))
}

#[patch("/user/daily_records/{recorded_on}")]
async fn update(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    recorded_on: Path<NaiveDate>,
    form: Json<UpdateDailyRecord>,
) -> MyResult<Json<()>> {
    let user_id = at.into_inner().id;
    let recorded_on = recorded_on.into_inner();
    let form = form.into_inner();

    let daily_record = DailyRecord::find_by(&**pool, user_id.clone(), recorded_on).await?;
    match daily_record {
        Some(mut daily_record) => {
            daily_record.can_write(user_id.clone())?;
            daily_record.update(form);
            daily_record.upsert(&**pool).await?;
            Ok(Json(()))
        }
        None => {
            let habits = THabit::find_many_by(&**pool, user_id.clone(), recorded_on).await?;
            let mut daily_record = DailyRecord::create(
                form.comment.clone(),
                recorded_on,
                user_id,
                habits.into_iter().map(|v| v.id).collect(),
            );
            daily_record.update(form);
            daily_record.upsert(&**pool).await?;
            Ok(Json(()))
        }
    }
}
