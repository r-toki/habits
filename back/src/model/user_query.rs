use crate::lib::my_error::MyResult;

use serde::Serialize;
use sqlx::{query_as, MySqlPool};

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
}

pub async fn find(pool: &MySqlPool, id: String) -> MyResult<UserDto> {
    query_as!(
        UserDto,
        r#"
select id, name from users
where id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}
