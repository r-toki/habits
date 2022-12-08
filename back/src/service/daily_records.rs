use crate::lib::Error;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct DailyRecord {
    comment: String,
    recorded_on: NaiveDate,
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
pub struct GetHabits {
    tz: Option<String>,
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
    recorded_on: String,
    query: GetHabits,
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
