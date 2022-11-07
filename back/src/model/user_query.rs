use crate::lib::my_error::MyResult;

use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
}

pub async fn find_by_id(pool: &PgPool, id: String) -> MyResult<Option<UserDto>> {
    query_as!(
        UserDto,
        r#"
select id, name from users
where id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}
