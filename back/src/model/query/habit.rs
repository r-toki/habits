use crate::lib::my_error::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDto {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    archived_at: Option<DateTime<Utc>>,
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
        "
        select id, name, created_at, updated_at, archived_at from habits
        where user_id = $1
        and ($2::bool is null or (case when $2 then archived_at is not null else archived_at is null end))
        order by sort_number
        ",
        user_id,
        habit_query.archived
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}
