use super::table;
use crate::lib::my_error::*;
use crate::model::lib::*;

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::query_as;

table! {
    "daily_records",
    pub struct TDailyRecord {
        pub id: String,
        pub comment: String,
        pub recorded_on: NaiveDate,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub user_id: String,
    }
}

impl TDailyRecord {
    pub fn create(
        comment: String,
        recorded_on: NaiveDate,
        user_id: String,
    ) -> MyResult<TDailyRecord> {
        let id = get_new_id();
        let now = get_current_date_time();
        let daily_record = TDailyRecord::new(id, comment, recorded_on, now, now, user_id);
        daily_record.validate()?;
        Ok(daily_record)
    }

    pub fn update(&mut self, comment: String) -> MyResult<()> {
        self.comment = comment;
        self.updated_at = get_current_date_time();
        self.validate()?;
        Ok(())
    }

    pub fn validate(&self) -> MyResult<()> {
        if self.comment.len() == 0 {
            return Err(MyError::UnprocessableEntity(
                "comment must be at least 1 character".into(),
            ));
        }
        Ok(())
    }
}

impl TDailyRecord {
    pub async fn find_by(
        executor: impl PgExecutor<'_>,
        user_id: String,
        recorded_on: NaiveDate,
    ) -> MyResult<Option<TDailyRecord>> {
        query_as!(
            TDailyRecord,
            r#"
select * from daily_records
where user_id = $1
and recorded_on = $2
            "#,
            user_id,
            recorded_on
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
