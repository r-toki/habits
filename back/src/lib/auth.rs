use crate::lib::{
    config::CONFIG,
    my_error::{MyError, MyResult},
};

use serde::Deserialize;

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

#[derive(Debug, Deserialize)]
pub struct AuthUser {
    pub uid: String,
}

pub async fn get_auth_user(access_token: String) -> MyResult<AuthUser> {
    let res = client()
        .get(format!("{}/me", CONFIG.auth_origin))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|_| MyError::new_unauthorized())?;

    let auth_user = res
        .json::<AuthUser>()
        .await
        .map_err(|_| MyError::new_unauthorized())?;

    Ok(auth_user)
}
