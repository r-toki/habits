use crate::lib::my_error::MyResult;

use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: String,
    pub display_name: String,
}

pub async fn find_user(pool: &PgPool, id: String) -> MyResult<UserDto> {
    query_as!(
        UserDto,
        r#"
select id, display_name from users
where id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}
