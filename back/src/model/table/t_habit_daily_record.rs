use super::table;
use crate::model::lib::*;

use chrono::{DateTime, NaiveDate, Utc};

/* ---------------------------------- Table --------------------------------- */
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

/* --------------------------------- Domain --------------------------------- */
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
}
