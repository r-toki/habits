use crate::lib::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    todo!();
}

pub async fn create_habit(pool: &PgPool, user_id: String, input: CreateHabit) -> Result<(), Error> {
    todo!();
}

pub async fn update_habit(
    pool: &PgPool,
    user_id: String,
    habit_id: String,
    input: UpdateHabit,
) -> Result<(), Error> {
    todo!();
}

pub async fn delete_habit(pool: &PgPool, user_id: String, habit_id: String) -> Result<(), Error> {
    todo!();
}

pub async fn swap_habits(pool: &PgPool, user_id: String, input: SwapHabits) -> Result<(), Error> {
    todo!();
}
