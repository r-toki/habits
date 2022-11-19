use super::lib::{get_current_date_time, get_new_id};
use crate::lib::my_error::{MyError, MyResult};

use chrono::{DateTime, Utc};
use derive_new::new;
use sqlx::{query, query_as, PgPool};
use validator::Validate;

#[derive(new, Debug, Validate)]
pub struct Habit {
    pub id: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
    pub user_id: String,
}

impl Habit {
    // NOTE: Domain Logic
    pub fn create(name: String, user_id: String) -> MyResult<Habit> {
        let id = get_new_id();
        let now = get_current_date_time();

        let habit = Self::new(id, name, now, now, None, user_id);
        habit.validate()?;

        Ok(habit)
    }

    pub fn archive(&mut self) -> MyResult<()> {
        match self.archived_at {
            Some(_) => Err(MyError::UnprocessableEntity("already archived".into())),
            None => {
                self.archived_at = Some(get_current_date_time());
                Ok(())
            }
        }
    }

    pub fn unarchive(&mut self) -> MyResult<()> {
        match self.archived_at {
            Some(_) => {
                self.archived_at = None;
                Ok(())
            }
            None => Err(MyError::UnprocessableEntity("already unarchived".into())),
        }
    }

    // NOTE: Policy Logic
    pub fn can_write(&self, user_id: String) -> MyResult<()> {
        if self.user_id != user_id {
            return Err(MyError::new_forbidden());
        }
        Ok(())
    }

    // NOTE: Persistence Logic
    pub async fn find(pool: &PgPool, id: String) -> MyResult<Habit> {
        query_as!(
            Habit,
            r#"
select id, name, created_at, updated_at, archived_at, user_id from habits
where id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn store(&self, pool: &PgPool) -> MyResult<()> {
        query!(
            r#"
insert into habits (id, name, created_at, updated_at, archived_at, user_id)
values ($1, $2, $3, $4, $5, $6)
on conflict (id)
do update
set name = $2, created_at = $3, updated_at = $4, archived_at = $5, user_id = $6
            "#,
            self.id,
            self.name,
            self.created_at,
            self.updated_at,
            self.archived_at,
            self.user_id
        )
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn delete(&self, pool: &PgPool) -> MyResult<()> {
        query!(
            r#"
delete from habits
where id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
}
