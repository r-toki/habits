use super::lib::get_current_date_time;
use crate::lib::my_error::MyResult;

use chrono::{DateTime, Utc};
use derive_new::new;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::{query, PgPool};
use validator::Validate;

lazy_static! {
    static ref RE_NAME: Regex = Regex::new(r"[A-Za-z\d#$@!%&*?]{3,15}").unwrap();
}

#[derive(new, Debug, Validate)]
pub struct User {
    pub id: String,
    #[validate(regex(
        path = "RE_NAME",
        message = "must be 3-15 characters in alphabet, numbers or symbols"
    ))]
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn create(id: String, name: String) -> MyResult<Self> {
        let now = get_current_date_time();

        let user = User::new(id, name, now, now);
        user.validate()?;

        Ok(user)
    }

    pub async fn store(&self, pool: &PgPool) -> MyResult<()> {
        query!(
            r#"
insert into users (id, name, created_at, updated_at)
values ($1, $2, $3, $4)
on conflict (id)
do update
set name = $2, created_at = $3, updated_at = $4
            "#,
            self.id,
            self.name,
            self.created_at,
            self.updated_at
        )
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
}
