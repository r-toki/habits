use super::lib::get_new_id;
use crate::lib::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    id: String,
    name: String,
    archived: bool,
    created_at: DateTime<Utc>,
    recent_done_list: Vec<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHabits {
    tz: Option<String>,
    archived: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHabit {
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHabit {
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapHabits {
    habit_id_1: String,
    habit_id_2: String,
}

pub async fn get_habits(
    pool: &PgPool,
    user_id: String,
    query: GetHabits,
) -> Result<Vec<Habit>, Error> {
    query_as!(
        Habit,
        r#"
        select
            habits.id,
            habits.name,
            habits.archived_at is not null "archived!",
            habits.created_at,
            array_agg(coalesce(habit_daily_records.done, false)) "recent_done_list!"
        from
            (
                select
                    *
                from
                    habits,
                    (
                        select
                            generate_series(
                                (current_date at time zone $1)::date - interval '6 days',
                                (current_date at time zone $1)::date,
                                '1 day'
                            )::date d
                    ) last_days
                where habits.created_at < (last_days.d::timestamp at time zone $1) + interval '1 day'
                    and user_id = $2
                    and ($3::bool is null or (case when $3 then archived_at is not null else archived_at is null end))
            ) habits
        left outer join habit_daily_records
        on habits.id = habit_daily_records.habit_id
        and habits.d = habit_daily_records.recorded_on
        group by habits.id, habits.name, habits.archived_at, habits.created_at, habits.sort_number
        order by habits.sort_number
        "#,
        match query.tz {
            Some(tz) => tz,
            None => "UTC".into(),
        },
        user_id,
        query.archived
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

pub async fn create_habit(pool: &PgPool, user_id: String, input: CreateHabit) -> Result<(), Error> {
    query!(
        "
        insert into habits (id, name, created_at, updated_at, archived_at, user_id, sort_number)
        values ($1, $2, current_timestamp, current_timestamp, null, $3, extract(epoch from current_timestamp)::int)
        ",
        get_new_id(),
        input.name,
        user_id
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(Into::into)
}

pub async fn update_habit(
    pool: &PgPool,
    user_id: String,
    habit_id: String,
    input: UpdateHabit,
) -> Result<(), Error> {
    query!(
        "
        update habits
        set
            name = $1,
            updated_at = current_timestamp
        where id = $2
        and user_id = $3
        ",
        input.name,
        habit_id,
        user_id
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(Into::into)
}

pub async fn delete_habit(pool: &PgPool, user_id: String, habit_id: String) -> Result<(), Error> {
    query!(
        "
        delete from habits
        where id = $1
        and user_id = $2
        ",
        habit_id,
        user_id
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(Into::into)
}

pub async fn archive_habit(pool: &PgPool, user_id: String, habit_id: String) -> Result<(), Error> {
    query!(
        "
        update habits
        set archived_at = current_timestamp
        where id = $1
        and user_id = $2
        and archived_at is null
        ",
        habit_id,
        user_id
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(Into::into)
}

pub async fn swap_habits(pool: &PgPool, user_id: String, input: SwapHabits) -> Result<(), Error> {
    query!(
        "
        update habits
        set sort_number =
            case id
                when $1 then (select sort_number from habits where id = $2)
                when $2 then (select sort_number from habits where id = $1)
            end
        where id in ($1, $2)
        and user_id = $3
        ",
        input.habit_id_1,
        input.habit_id_2,
        user_id
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(Into::into)
}
