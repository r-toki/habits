use crate::lib::my_error::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDto {
    id: String,
    name: String,
    archived: bool,
    created_at: DateTime<Utc>,
    recent_done_list: Vec<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FindHabitsQuery {
    archived: Option<bool>,
}

pub async fn find_habits(
    pool: &PgPool,
    user_id: String,
    habit_query: FindHabitsQuery,
) -> MyResult<Vec<HabitDto>> {
    query_as!(
        HabitDto,
        r#"
        select
        habits.id, habits.name, habits.archived_at is not null "archived!", habits.created_at, array_agg(coalesce(habit_daily_records.done, false)) "recent_done_list!"
        from
        (
            select
            *
            from
            habits,
            (
                select
                generate_series(
                    (current_date::date at time zone 'Asia/Tokyo')::timestamp - interval '5 days',
                    (current_date::date at time zone 'Asia/Tokyo')::timestamp + interval '1 days',
                    '1 day'
                ) _timestamp
            ) last_days
            where habits.created_at < last_days._timestamp
            and user_id = $1
            and ($2::bool is null or (case when $2 then archived_at is not null else archived_at is null end))
        ) habits
        left outer join habit_daily_records
        on habits.id = habit_daily_records.habit_id
        and habits._timestamp::date = habit_daily_records.recorded_on
        group by habits.id, habits.name, habits.archived_at, habits.created_at, habits.sort_number
        order by habits.sort_number
        "#,
        user_id,
        habit_query.archived
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}
