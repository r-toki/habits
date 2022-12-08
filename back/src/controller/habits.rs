use super::lib::AccessTokenDecoded;
use crate::lib::Error;
use crate::service::habits::*;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, Query, ServiceConfig},
};
use sqlx::PgPool;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(create_archive);
    cfg.service(create_swap);
}

#[get("/habits")]
async fn index(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    query: Query<GetHabits>,
) -> Result<Json<Vec<Habit>>, Error> {
    get_habits(&**pool, at.into_inner().uid, query.into_inner())
        .await
        .map(Json)
}

#[post("/habits")]
async fn create(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    form: Json<CreateHabit>,
) -> Result<Json<()>, Error> {
    create_habit(&**pool, at.into_inner().uid, form.into_inner())
        .await
        .map(Json)
}

#[patch("/habits/{habit_id}")]
async fn update(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<String>,
    form: Json<UpdateHabit>,
) -> Result<Json<()>, Error> {
    update_habit(
        &**pool,
        at.into_inner().uid,
        path.into_inner(),
        form.into_inner(),
    )
    .await
    .map(Json)
}

#[delete("/habits/{habit_id}")]
async fn delete(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<String>,
) -> Result<Json<()>, Error> {
    delete_habit(&**pool, at.into_inner().uid, path.into_inner())
        .await
        .map(Json)
}

#[post("/habits/{habit_id}/archive")]
async fn create_archive(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    path: Path<String>,
) -> Result<Json<()>, Error> {
    archive_habit(&**pool, at.into_inner().uid, path.into_inner())
        .await
        .map(Json)
}

#[post("/habits/swap")]
async fn create_swap(
    pool: Data<PgPool>,
    at: AccessTokenDecoded,
    form: Json<SwapHabits>,
) -> Result<Json<()>, Error> {
    swap_habits(&**pool, at.into_inner().uid, form.into_inner())
        .await
        .map(Json)
}
