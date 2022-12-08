use super::lib::get_new_id;
use crate::lib::Error;
use chrono::NaiveDate;
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};

#[derive(new, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyRecord {
    comment: String,
    recorded_on: NaiveDate,
    habit_daily_records: Vec<HabitDailyRecord>,
}

#[derive(new, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDailyRecord {
    done: bool,
    archived: bool,
    habit_id: String,
    habit_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDailyRecord {
    tz: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDailyRecord {
    comment: String,
    habit_daily_records: Vec<UpdateHabitDailyRecord>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHabitDailyRecord {
    done: bool,
    habit_id: String,
}

pub async fn get_daily_record(
    pool: &PgPool,
    user_id: String,
    recorded_on: NaiveDate,
    q: GetDailyRecord,
) -> Result<Option<DailyRecord>, Error> {
    let daily_record = query!(
        "
        select comment, recorded_on from daily_records
        where user_id = $1
        and recorded_on = $2
        ",
        user_id,
        recorded_on
    )
    .fetch_optional(pool)
    .await?;

    let habit_daily_records: Vec<HabitDailyRecord> = query!(
        r#"
        select
            hdr.done "done?", h.archived_at is not null "archived!", h.id habit_id, h.name habit_name
        from
            (
                select * from habits
                where user_id = $1
                and (archived_at is null or archived_at > $2::date::timestamp at time zone $3)
                and created_at < $2::date::timestamp at time zone $3 + interval '1 day'
                order by sort_number
            ) h
        left outer join
            (
                select * from habit_daily_records
                where recorded_on = $2
            ) hdr
        on h.id = hdr.habit_id
        "#,
        user_id,
        recorded_on,
        q.tz
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|v| {
        HabitDailyRecord::new(
            v.done.unwrap_or(false),
            v.archived,
            v.habit_id,
            v.habit_name,
        )
    })
    .collect();

    if habit_daily_records.len() == 0 {
        return Ok(None);
    }

    Ok(Some(DailyRecord::new(
        daily_record.map(|v| v.comment).unwrap_or("".into()),
        recorded_on,
        habit_daily_records,
    )))
}

pub async fn update_daily_record(
    pool: &PgPool,
    user_id: String,
    recorded_on: NaiveDate,
    f: UpdateDailyRecord,
) -> Result<(), Error> {
    let mut tx = pool.begin().await?;

    let daily_record = query!(
        "
        insert into daily_records (id, comment, recorded_on, created_at, updated_at, user_id)
        values ($1, $2, $3, current_timestamp, current_timestamp, $4)
        on conflict (recorded_on, user_id)
        do update
        set comment = $2, updated_at = current_timestamp
        returning *
        ",
        get_new_id(),
        f.comment,
        recorded_on,
        user_id
    )
    .fetch_one(&mut tx)
    .await?;

    for habit_daily_record in f.habit_daily_records.into_iter() {
        query!(
            "
            insert into habit_daily_records (id, done, recorded_on, created_at, updated_at, habit_id, daily_record_id)
            values ($1, $2, $3, current_timestamp, current_timestamp, $4, $5)
            on conflict (recorded_on, habit_id, daily_record_id)
            do update
            set done = $2, updated_at = current_timestamp
            ",
            get_new_id(),
            habit_daily_record.done,
            recorded_on,
            habit_daily_record.habit_id,
            daily_record.id
        )
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
