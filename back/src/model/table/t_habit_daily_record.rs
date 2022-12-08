use super::table;
use crate::lib::error::*;
use crate::model::lib::*;

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::query_as;

table! {
    "habit_daily_records",
    pub struct THabitDailyRecord {
        pub id: String,
        pub done: bool,
        pub recorded_on: NaiveDate,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub habit_id: String,
        pub daily_record_id: String,
    }
}

impl THabitDailyRecord {
    pub fn create(
        recoded_on: NaiveDate,
        habit_id: String,
        daily_record_id: String,
    ) -> THabitDailyRecord {
        let id = get_new_id();
        let now = get_current_date_time();
        THabitDailyRecord::new(id, false, recoded_on, now, now, habit_id, daily_record_id)
    }

    pub fn update(&mut self, done: bool) {
        self.done = done;
        self.updated_at = get_current_date_time();
    }
}

impl THabitDailyRecord {
    pub async fn many_of_daily_record(
        executor: impl PgExecutor<'_>,
        daily_record_id: String,
    ) -> MyResult<Vec<THabitDailyRecord>> {
        query_as!(
            THabitDailyRecord,
            "
            select * from habit_daily_records
            where daily_record_id = $1
            ",
            daily_record_id
        )
        .fetch_all(executor)
        .await
        .map_err(Into::into)
    }
}
