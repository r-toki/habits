use crate::{lib::my_error::MyResult, model::lib::get_new_id};

use chrono::{DateTime, NaiveDate, Utc};
use derive_new::new;
use sqlx::{query, query_as, PgPool};

use super::lib::get_current_date_time;

#[derive(new, Debug)]
pub struct DailyRecord {
    id: String,
    comment: String,
    recorded_on: NaiveDate,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_id: String,
    habit_daily_records: Vec<HabitDailyRecord>,
}

#[derive(new, Debug)]
pub struct HabitDailyRecord {
    id: String,
    done: bool,
    recorded_on: NaiveDate,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    habit_id: String,
    daily_record_id: String,
}

impl DailyRecord {
    pub fn create(recorded_on: NaiveDate, user_id: String, habit_ids: Vec<String>) -> DailyRecord {
        let id = get_new_id();
        let now = get_current_date_time();
        let h_d_rs: Vec<HabitDailyRecord> = habit_ids
            .into_iter()
            .map(|h_id| HabitDailyRecord::create(recorded_on, h_id, id.clone()))
            .collect();
        DailyRecord::new(id, "".into(), recorded_on, now, now, user_id, h_d_rs)
    }

    pub async fn find_by(
        pool: &PgPool,
        recorded_on: NaiveDate,
        user_id: String,
    ) -> MyResult<Option<DailyRecord>> {
        let raw_d_r = query!(
            r#"
select * from daily_records
where recorded_on = $1
and user_id = $2
            "#,
            recorded_on,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        match raw_d_r {
            Some(raw_d_r) => {
                let h_d_rs = query_as!(
                    HabitDailyRecord,
                    r#"
select * from habit_daily_records
where daily_record_id = $1
                    "#,
                    raw_d_r.id
                )
                .fetch_all(pool)
                .await?;

                let daily_record = DailyRecord::new(
                    raw_d_r.id,
                    raw_d_r.comment,
                    raw_d_r.recorded_on,
                    raw_d_r.created_at,
                    raw_d_r.updated_at,
                    raw_d_r.user_id,
                    h_d_rs,
                );

                Ok(Some(daily_record))
            }
            None => Ok(None),
        }
    }

    pub async fn store(&self, pool: &PgPool) -> MyResult<()> {
        let mut tx = pool.begin().await?;

        query!(
            r#"
insert into daily_records (id, comment, recorded_on, created_at, updated_at, user_id)
values ($1, $2, $3, $4, $5, $6)
on conflict (id)
do update
set comment = $2, recorded_on = $3, created_at = $4, updated_at = $5, user_id = $6
            "#,
            self.id,
            self.comment,
            self.recorded_on,
            self.created_at,
            self.updated_at,
            self.user_id
        )
        .execute(&mut tx)
        .await?;

        for h_d_r in self.habit_daily_records.iter() {
            query!(
                r#"
insert into habit_daily_records (id, done, recorded_on, created_at, updated_at, habit_id, daily_record_id)
values ($1, $2, $3, $4, $5, $6, $7)
on conflict (id)
do update
set done = $2, recorded_on = $3, created_at = $4, updated_at = $5, habit_id = $6, daily_record_id = $7
                "#,
                h_d_r.id,
                h_d_r.done,
                h_d_r.recorded_on,
                h_d_r.created_at,
                h_d_r.updated_at,
                h_d_r.habit_id,
                h_d_r.daily_record_id
            )
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

impl HabitDailyRecord {
    pub fn create(
        recorded_on: NaiveDate,
        habit_id: String,
        daily_record_id: String,
    ) -> HabitDailyRecord {
        let id = get_new_id();
        let now = get_current_date_time();
        HabitDailyRecord::new(id, false, recorded_on, now, now, habit_id, daily_record_id)
    }
}
