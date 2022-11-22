use crate::lib::my_error::*;
use crate::model::table::*;

use derive_new::new;
use serde::Serialize;
use sqlx::PgPool;

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    id: String,
    display_name: String,
}

pub async fn find_user(pool: &PgPool, id: String) -> MyResult<UserDto> {
    TUser::find(pool, id)
        .await
        .map(|u| UserDto::new(u.id, u.display_name))
        .map_err(Into::into)
}
