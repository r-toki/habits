use super::*;

/* ---------------------------------- Table --------------------------------- */
table! {
    "habits",
    pub struct THabit {
        pub id: String,
        pub name: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub archived_at: Option<DateTime<Utc>>,
        pub user_id: String,
    }
}

/* --------------------------------- Command -------------------------------- */
impl THabit {
    pub fn create(name: String, user_id: String) -> MyResult<THabit> {
        if name.len() == 0 {
            return Err(unprocessable_entity(
                "habit name must be at leas 1 character",
            ));
        }
        let id = get_new_id();
        let now = get_current_date_time();
        let habit = THabit::new(id, name, now, now, None, user_id);
        Ok(habit)
    }

    pub fn archive(&mut self) -> MyResult<()> {
        match self.archived_at {
            Some(_) => Err(unprocessable_entity("habit is already archived")),
            None => {
                self.archived_at = Some(get_current_date_time());
                Ok(())
            }
        }
    }

    pub fn can_write(&self, user_id: String) -> MyResult<()> {
        if self.user_id != user_id {
            return Err(forbidden("can not write habit"));
        }
        Ok(())
    }
}

/* ---------------------------------- Query --------------------------------- */
#[derive(Debug, Deserialize)]
pub struct HabitQuery {
    archived: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDto {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    archived_at: Option<DateTime<Utc>>,
}

pub async fn find_habits(
    pool: &PgPool,
    user_id: String,
    habit_query: HabitQuery,
) -> MyResult<Vec<HabitDto>> {
    query_as!(
        HabitDto,
        r#"
select id, name, created_at, updated_at, archived_at from habits
where user_id = $1
and ($2::bool is null or (case when $2 then archived_at is not null else archived_at is null end))
order by created_at
        "#,
        user_id,
        habit_query.archived
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}
