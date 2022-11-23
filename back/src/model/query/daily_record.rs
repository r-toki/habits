use crate::lib::my_error::*;
use crate::model::lib::*;
use crate::model::table::*;

use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use derive_new::new;
use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyRecordDto {
    pub id: String,
    pub comment: String,
    pub recorded_on: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub habit_daily_records: Vec<HabitDailyRecordDto>,
}

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDailyRecordDto {
    pub id: String,
    pub done: bool,
    pub recorded_on: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

pub async fn find_daily_record(
    pool: &PgPool,
    user_id: String,
    recorded_on: NaiveDate,
) -> MyResult<DailyRecordDto> {
    let t_daily_record = query_as!(
        TDailyRecord,
        r#"
select * from daily_records
where user_id = $1
and recorded_on = $2
        "#,
        user_id,
        recorded_on
    )
    .fetch_optional(pool)
    .await?;

    match t_daily_record {
        Some(t_daily_record) => {
            let habit_daily_records_dto = query_as!(
                HabitDailyRecordDto,
                r#"
select
    h_d_r.id id,
    h_d_r.done done,
    h_d_r.recorded_on recorded_on,
    h_d_r.created_at created_at,
    h_d_r.updated_at updated_at,
    h.name name
from
    habit_daily_records h_d_r
    inner join habits h
    on h_d_r.habit_id = h.id
where daily_record_id = $1
                "#,
                t_daily_record.id.clone()
            )
            .fetch_all(pool)
            .await?;

            let habit_record_dto = DailyRecordDto::new(
                t_daily_record.id,
                t_daily_record.comment,
                t_daily_record.recorded_on,
                t_daily_record.created_at,
                t_daily_record.updated_at,
                habit_daily_records_dto,
            );

            Ok(habit_record_dto)
        }

        None => {
            let offset = FixedOffset::east_opt(9 * 60 * 60).unwrap();
            let recorded_at_start_of_day =
                DateTime::<FixedOffset>::from_local(recorded_on.and_hms(0, 0, 0), offset);
            let recorded_at_end_of_day =
                DateTime::<FixedOffset>::from_local(recorded_on.and_hms(23, 59, 59), offset);

            let t_habits = query_as!(
                THabit,
                r#"
select * from habits
where user_id = $1
and (archived_at is null or archived_at > $2)
and created_at < $3
order by created_at
                "#,
                user_id,
                recorded_at_start_of_day,
                recorded_at_end_of_day
            )
            .fetch_all(pool)
            .await?;

            let now = get_current_date_time();

            let habit_daily_records_dto: Vec<HabitDailyRecordDto> = t_habits
                .into_iter()
                .map(|t_habit| {
                    HabitDailyRecordDto::new(
                        get_new_id(),
                        false,
                        recorded_on,
                        now,
                        now,
                        t_habit.name,
                    )
                })
                .collect();

            let daily_record_dto = DailyRecordDto::new(
                get_new_id(),
                "".into(),
                recorded_on,
                now,
                now,
                habit_daily_records_dto,
            );

            Ok(daily_record_dto)
        }
    }
}
