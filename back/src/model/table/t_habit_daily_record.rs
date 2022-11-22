use super::*;

/* ---------------------------------- Table --------------------------------- */
table! {
    "habit_daily_records",
    pub struct THabitDailyRecord {
        id: String,
        done: bool,
        recorded_on: NaiveDate,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        habit_id: String,
        daily_record_id: String,
    }
}
