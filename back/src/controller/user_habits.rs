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
    let habit = THabit::create(form.name.clone(), at.into_inner().id)?;
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
    let mut habit = THabit::find(&**pool, path.into_inner()).await?;
    habit.can_write(at.into_inner().id)?;
    habit.archive()?;
    habit.upsert(&**pool).await?;
    Ok(Json(()))
}
