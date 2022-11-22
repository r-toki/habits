use super::*;

/* ---------------------------------- Table --------------------------------- */
table! {
    "daily_records",
    pub struct TDailyRecord {
        id: String,
        comment: String,
        recorded_on: NaiveDate,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }
}
