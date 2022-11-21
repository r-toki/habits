use crate::controller::lib::jwt_extractor::AccessTokenDecoded;
use crate::lib::my_error::MyResult;
use crate::model::table::*;

use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
    cfg.service(habits_index);
    cfg.service(create_habit);
    cfg.service(delete_habit);
    cfg.service(create_habit_archive);
}

#[get("/user")]
async fn index(pool: Data<PgPool>, at: AccessTokenDecoded) -> MyResult<Json<UserDto>> {
    let user = find_user(&**pool, at.into_inner().id).await?;
    Ok(Json(user))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Create {
    display_name: String,
}

#[post("/user")]
async fn create(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    form: Json<Create>,
) -> MyResult<Json<()>> {
    let user = TUser::create(at.into_inner().id, form.display_name.clone())?;
    user.upsert(&**pool).await?;
    Ok(Json(()))
}

#[get("/user/habits")]
async fn habits_index(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    query: Query<HabitQuery>,
) -> MyResult<Json<Vec<HabitDto>>> {
    let habits = find_habits(&**pool, at.into_inner().id, query.into_inner()).await?;
    Ok(Json(habits))
}

#[derive(Debug, Deserialize)]
struct CreateHabit {
    name: String,
}

#[post("/user/habits")]
async fn create_habit(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    form: Json<CreateHabit>,
) -> MyResult<Json<()>> {
    let habit = THabit::create(form.name.clone(), at.into_inner().id)?;
    habit.upsert(&**pool).await?;
    Ok(Json(()))
}

#[delete("/user/habits/{habit_id}")]
async fn delete_habit(
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
async fn create_habit_archive(
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
