use crate::controller::lib::*;
use crate::lib::my_error::*;
use crate::model::{query::*, table::*};

use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
    cfg.service(delete);
    cfg.service(create_archive);
    cfg.service(create_swap);
}

#[get("/user/habits")]
async fn index(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    query: Query<FindHabitsQuery>,
) -> MyResult<Json<Vec<HabitDto>>> {
    let habits = find_habits(&**pool, at.into_inner().id, query.into_inner()).await?;
    Ok(Json(habits))
}

#[derive(Debug, Deserialize)]
struct Create {
    name: String,
}

#[post("/user/habits")]
async fn create(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    form: Json<Create>,
) -> MyResult<Json<()>> {
    let user_id = at.into_inner().id;
    let habit = THabit::create(form.name.clone(), user_id.clone())?;
    habit.upsert(&**pool).await?;
    Ok(Json(()))
}

#[delete("/user/habits/{habit_id}")]
async fn delete(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<String>,
) -> MyResult<Json<()>> {
    let habit = THabit::find(&**pool, path.into_inner()).await?;
    habit.can_write(at.into_inner().id)?;
    habit.delete(&**pool).await?;
    Ok(Json(()))
}

#[post("/user/habits/{habit_id}/archive")]
async fn create_archive(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<String>,
) -> MyResult<Json<()>> {
    let mut habit = THabit::one_of_user_by_id(&**pool, at.into_inner().id, path.into_inner())
        .await?
        .ok_or(MyError::new_not_found())?;
    habit.archive()?;
    habit.upsert(&**pool).await?;
    Ok(Json(()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSwap {
    habit_id_1: String,
    habit_id_2: String,
}

#[post("/user/habits/swap")]
async fn create_swap(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    form: Json<CreateSwap>,
) -> MyResult<Json<()>> {
    let user_id = at.into_inner().id;
    let mut habit_1 = THabit::one_of_user_by_id(&**pool, user_id.clone(), form.habit_id_1.clone())
        .await?
        .ok_or(MyError::new_not_found())?;
    let mut habit_2 = THabit::one_of_user_by_id(&**pool, user_id.clone(), form.habit_id_2.clone())
        .await?
        .ok_or(MyError::new_not_found())?;
    let habit_1_sort_number = habit_1.sort_number;
    habit_1.update_sort_number(habit_2.sort_number)?;
    habit_2.update_sort_number(habit_1_sort_number)?;
    habit_1.upsert(&**pool).await?;
    habit_2.upsert(&**pool).await?;
    Ok(Json(()))
}
