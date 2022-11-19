use crate::lib::my_error::MyResult;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Debug, Deserialize)]
pub struct HabitQuery {
    archived: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDto {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    archived_at: Option<DateTime<Utc>>,
}

pub async fn find_habits(
    pool: &PgPool,
    user_id: String,
    habit_query: HabitQuery,
) -> MyResult<Vec<HabitDto>> {
    query_as!(
        HabitDto,
        r#"
select id, name, created_at, updated_at, archived_at from habits
where user_id = $1
and $2::bool is null or case when $2 then archived_at is not null else archived_at is null end
order by created_at
        "#,
        user_id,
        habit_query.archived
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}
