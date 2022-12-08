use crate::lib::Error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    todo!();
}

pub async fn create_me(pool: &PgPool, user_id: String, input: CreateMe) -> Result<(), Error> {
    todo!();
}
