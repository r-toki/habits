use crate::lib::Error;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Me {
    id: String,
    display_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMe {
    display_name: String,
}

pub async fn get_me(pool: &PgPool, user_id: String) -> Result<Me, Error> {
    query_as!(
        Me,
        "
        select id, display_name from users
        where id = $1
        ",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

pub async fn create_me(pool: &PgPool, user_id: String, f: CreateMe) -> Result<(), Error> {
    query!(
        "
        insert into users (id, display_name, created_at, updated_at)
        values ($1, $2, current_timestamp, current_timestamp)
        ",
        user_id,
        f.display_name
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(Into::into)
}
