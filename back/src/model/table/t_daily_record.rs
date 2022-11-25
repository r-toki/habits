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
    pub fn create(comment: String, recorded_on: NaiveDate, user_id: String) -> TDailyRecord {
        let id = get_new_id();
        let now = get_current_date_time();
        TDailyRecord::new(id, comment, recorded_on, now, now, user_id)
    }

    pub fn update(&mut self, comment: String) {
        self.comment = comment;
        self.updated_at = get_current_date_time();
    }
}

impl TDailyRecord {
    pub async fn one_of_user_by_recorded_on(
        executor: impl PgExecutor<'_>,
        user_id: String,
        recorded_on: NaiveDate,
    ) -> MyResult<Option<TDailyRecord>> {
        query_as!(
            TDailyRecord,
            "
            select * from daily_records
            where user_id = $1
            and recorded_on = $2
            ",
            user_id,
            recorded_on
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
