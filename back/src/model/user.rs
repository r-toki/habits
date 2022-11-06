use super::lib::get_current_date_time;
use crate::lib::my_error::MyResult;

use chrono::{DateTime, Utc};
use derive_new::new;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::{query, query_as, MySqlExecutor, MySqlPool};
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

    pub async fn find(pool: &MySqlPool, id: String) -> MyResult<User> {
        query_as!(
            User,
            r#"
select * from users
where id = ?
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn find_by_name(
        executor: impl MySqlExecutor<'_>,
        name: String,
    ) -> MyResult<Option<User>> {
        query_as!(
            User,
            r#"
select * from users
where name = ?
            "#,
            name
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }

    pub async fn delete_by_id(pool: &MySqlPool, id: String) -> MyResult<()> {
        query!(
            r#"
delete from users
where id = ?
            "#,
            id
        )
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn store(&self, pool: &MySqlPool) -> MyResult<()> {
        let mut tx = pool.begin().await?;

        let exists = query!(
            r#"
select * from users
where id = ?
            "#,
            self.id
        )
        .fetch_optional(&mut tx)
        .await?;

        match exists {
            Some(_) => {
                query!(
                    r#"
update users
set name = ?, created_at = ?, updated_at = ?
where id = ?
                    "#,
                    self.name,
                    self.created_at,
                    self.updated_at,
                    self.id
                )
                .execute(&mut tx)
                .await?;
            }
            None => {
                query!(
                    r#"
insert into users (id, name, created_at, updated_at)
values (?, ?, ?, ?)
                    "#,
                    self.id,
                    self.name,
                    self.created_at,
                    self.updated_at
                )
                .execute(&mut tx)
                .await?;
            }
        };

        tx.commit().await?;

        Ok(())
    }
}
