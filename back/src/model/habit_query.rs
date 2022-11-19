use crate::lib::my_error::MyResult;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDto {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
}

pub async fn find_habits(pool: &PgPool, user_id: String) -> MyResult<Vec<HabitDto>> {
    query_as!(
        HabitDto,
        r#"
select id, name, created_at, updated_at, archived_at from habits
where user_id = $1
order by created_at
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}
