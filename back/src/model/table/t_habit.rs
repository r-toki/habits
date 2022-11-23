use super::table;
use crate::lib::my_error::*;
use crate::model::lib::*;

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::query_as;

table! {
    "habits",
    pub struct THabit {
        pub id: String,
        pub name: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub archived_at: Option<DateTime<Utc>>,
        pub user_id: String,
    }
}

impl THabit {
    pub fn create(name: String, user_id: String) -> MyResult<THabit> {
        if name.len() == 0 {
            return Err(MyError::UnprocessableEntity(
                "habit name must be at leas 1 character".into(),
            ));
        }
        let id = get_new_id();
        let now = get_current_date_time();
        let habit = THabit::new(id, name, now, now, None, user_id);
        Ok(habit)
    }

    pub fn archive(&mut self) -> MyResult<()> {
        match self.archived_at {
            Some(_) => Err(MyError::UnprocessableEntity(
                "habit is already archived".into(),
            )),
            None => {
                self.archived_at = Some(get_current_date_time());
                Ok(())
            }
        }
    }

    pub fn can_write(&self, user_id: String) -> MyResult<()> {
        if self.user_id != user_id {
            return Err(MyError::Forbidden("can not write habit".into()));
        }
        Ok(())
    }
}

impl THabit {
    pub async fn find_many_by(
        executor: impl PgExecutor<'_>,
        user_id: String,
        recorded_on: NaiveDate,
    ) -> MyResult<Vec<THabit>> {
        query_as!(
            THabit,
            r#"
select * from habits
where user_id = $1
and (archived_at is null or archived_at > $2)
and created_at < $3
order by created_at
            "#,
            user_id,
            start_of_date(recorded_on),
            end_of_date(recorded_on)
        )
        .fetch_all(executor)
        .await
        .map_err(Into::into)
    }
}
