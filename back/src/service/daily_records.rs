use crate::lib::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct DailyRecord {
    comment: String,
    start_of_recorded_at: DateTime<Utc>,
    habit_daily_records: Vec<HabitDailyRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDailyRecord {
    done: bool,
    archived: bool,
    habit_id: String,
    habit_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDailyRecord {
    comment: String,
    habit_daily_records: Vec<UpdateHabitDailyRecord>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHabitDailyRecord {
    done: bool,
    habit_id: String,
}

pub async fn get_daily_record(
    pool: &PgPool,
    user_id: String,
    start_of_recorded_at: DateTime<Utc>,
) -> Result<DailyRecord, Error> {
    todo!();
}

pub async fn update_daily_record(
    pool: &PgPool,
    user_id: String,
    start_of_recorded_at: DateTime<Utc>,
    input: UpdateDailyRecord,
) -> Result<DailyRecord, Error> {
    todo!();
}
