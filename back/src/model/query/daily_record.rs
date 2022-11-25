use crate::lib::my_error::*;
use crate::model::lib::*;
use crate::model::table::*;

use chrono::NaiveDate;
use derive_new::new;
use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyRecordDto {
    pub id: String,
    pub comment: String,
    pub recorded_on: NaiveDate,
    pub habit_daily_records: Vec<HabitDailyRecordDto>,
}

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDailyRecordDto {
    pub id: String,
    pub done: bool,
    pub habit_id: String,
    pub habit_name: String,
}

pub async fn find_daily_record(
    pool: &PgPool,
    user_id: String,
    recorded_on: NaiveDate,
) -> MyResult<DailyRecordDto> {
    todo!();
}
