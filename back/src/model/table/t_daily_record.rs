use super::table;
use crate::lib::my_error::*;
use crate::model::lib::*;

use chrono::{DateTime, NaiveDate, Utc};

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
        if comment.len() == 0 {
            return Err(MyError::UnprocessableEntity(
                "comment must be at least 1 character".into(),
            ));
        }
        let id = get_new_id();
        let now = get_current_date_time();
        Ok(TDailyRecord::new(
            id,
            comment,
            recorded_on,
            now,
            now,
            user_id,
        ))
    }
}
