use crate::lib::my_error::*;
use crate::model::table::*;

use chrono::NaiveDate;
use derive_new::new;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(new, Debug)]
pub struct DailyRecord {
    pub t_daily_record: TDailyRecord,
    pub t_habit_daily_records: Vec<THabitDailyRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDailyRecord {
    pub comment: String,
    pub habit_daily_records: Vec<UpdateHabitDailyRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHabitDailyRecord {
    pub id: String,
    pub done: bool,
    pub habit_id: String,
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

    pub fn update(&mut self, input: UpdateDailyRecord) -> MyResult<()> {
        self.t_daily_record.update(input.comment)?;

        for habit_daily_record in input.habit_daily_records.iter() {
            let target = self
                .t_habit_daily_records
                .iter_mut()
                .find(|v| v.id == habit_daily_record.id);

            match target {
                Some(target) => target.update(habit_daily_record.done),
                None => {}
            }
        }

        Ok(())
    }

    pub fn can_write(&self, user_id: String) -> MyResult<()> {
        if self.t_daily_record.user_id != user_id {
            return Err(MyError::Forbidden("can not write daily record".into()));
        }
        Ok(())
    }
}

impl DailyRecord {
    pub async fn find_by(
        pool: &PgPool,
        user_id: String,
        recorded_on: NaiveDate,
    ) -> MyResult<Option<DailyRecord>> {
        let t_daily_record = TDailyRecord::find_by(pool, user_id, recorded_on).await?;
        match t_daily_record {
            Some(t_daily_record) => {
                let t_habit_daily_records =
                    THabitDailyRecord::find_many_by(pool, t_daily_record.id.clone()).await?;
                let daily_record = DailyRecord::new(t_daily_record, t_habit_daily_records);
                Ok(Some(daily_record))
            }
            None => Ok(None),
        }
    }

    pub async fn upsert(&self, pool: &PgPool) -> MyResult<()> {
        let mut tx = pool.begin().await?;
        self.t_daily_record.upsert(&mut tx).await?;
        for t_habit_daily_record in self.t_habit_daily_records.iter() {
            t_habit_daily_record.upsert(&mut tx).await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
