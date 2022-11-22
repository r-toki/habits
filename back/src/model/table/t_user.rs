use super::*;

lazy_static! {
    static ref RE_DISPLAY_NAME: Regex = Regex::new(r"[A-Za-z\d#$@!%&*?]{3,15}").unwrap();
}

/* ---------------------------------- Table --------------------------------- */
table! {
    "users",
    pub struct TUser {
        pub id: String,
        pub display_name: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }
}

/* --------------------------------- Domain -------------------------------- */
impl TUser {
    pub fn create(id: String, display_name: String) -> MyResult<TUser> {
        if !RE_DISPLAY_NAME.is_match(&display_name) {
            return Err(MyError::UnprocessableEntity(
                "display_name must be 3-15 characters in in alphabet, numbers or symbols".into(),
            ));
        }
        let now = get_current_date_time();
        let user = TUser::new(id, display_name, now, now);
        Ok(user)
    }
}

/* ---------------------------------- Query --------------------------------- */
#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    id: String,
    display_name: String,
}

pub async fn find_user(pool: &PgPool, id: String) -> MyResult<UserDto> {
    TUser::find(pool, id)
        .await
        .map(|u| UserDto::new(u.id, u.display_name))
        .map_err(Into::into)
}
