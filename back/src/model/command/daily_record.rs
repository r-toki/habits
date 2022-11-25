use crate::lib::my_error::*;
use crate::model::lib::*;
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
    pub done: bool,
    pub habit_id: String,
}

impl DailyRecord {
    pub fn update(&mut self, input: UpdateDailyRecord) {
        self.t_daily_record.update(input.comment);

        for habit_daily_record in input.habit_daily_records.iter() {
            let target = self
                .t_habit_daily_records
                .iter_mut()
                .find(|v| v.habit_id == habit_daily_record.habit_id);

            match target {
                Some(target) => target.update(habit_daily_record.done),
                None => {}
            }
        }
    }
}

impl DailyRecord {
    pub async fn one_of_user_by_recorded_on(
        pool: &PgPool,
        user_id: String,
        recorded_on: NaiveDate,
    ) -> MyResult<DailyRecord> {
        let t_daily_record =
            TDailyRecord::one_of_user_by_recorded_on(pool, user_id.clone(), recorded_on).await?;

        match t_daily_record {
            Some(t_daily_record) => {
                let mut t_habit_daily_records =
                    THabitDailyRecord::many_of_daily_record(pool, t_daily_record.id.clone())
                        .await?;
                let t_habits =
                    THabit::many_of_user_by_record_on(pool, user_id.clone(), recorded_on).await?;

                let not_recorded_habit_ids = vec_diff(
                    t_habits.iter().map(|v| v.id.clone()).collect(),
                    t_habit_daily_records
                        .iter()
                        .map(|v| v.habit_id.clone())
                        .collect(),
                );
                let mut not_recorded_habit_daily_records: Vec<THabitDailyRecord> =
                    not_recorded_habit_ids
                        .into_iter()
                        .map(|v| {
                            THabitDailyRecord::create(recorded_on, v, t_daily_record.id.clone())
                        })
                        .collect();

                t_habit_daily_records.append(&mut not_recorded_habit_daily_records);
                Ok(DailyRecord::new(t_daily_record, t_habit_daily_records))
            }

            None => {
                let t_daily_record =
                    TDailyRecord::create("".into(), recorded_on.clone(), user_id.clone());
                let t_habit_daily_records: Vec<THabitDailyRecord> =
                    THabit::many_of_user_by_record_on(pool, user_id.clone(), recorded_on)
                        .await?
                        .into_iter()
                        .map(|v| {
                            THabitDailyRecord::create(recorded_on, v.id, t_daily_record.id.clone())
                        })
                        .collect();
                Ok(DailyRecord::new(t_daily_record, t_habit_daily_records))
            }
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
