use crate::lib::my_error::*;
use crate::model::table::*;

use chrono::NaiveDate;
use derive_new::new;

#[derive(new, Debug)]
pub struct DailyRecord {
    pub t_daily_record: TDailyRecord,
    pub t_habit_daily_records: Vec<THabitDailyRecord>,
}

impl DailyRecord {
    pub fn create(
        comment: String,
        recorded_on: NaiveDate,
        user_id: String,
        habit_ids: Vec<String>,
    ) -> MyResult<DailyRecord> {
        let t_daily_record = TDailyRecord::create(comment, recorded_on.clone(), user_id)?;
        let t_habit_daily_records: Vec<THabitDailyRecord> = habit_ids
            .into_iter()
            .map(|habit_id| {
                THabitDailyRecord::create(recorded_on.clone(), habit_id, t_daily_record.id.clone())
            })
            .collect();
        Ok(DailyRecord::new(t_daily_record, t_habit_daily_records))
    }
}
