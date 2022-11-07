use crate::lib::{
    config::CONFIG,
    my_error::{MyError, MyResult},
};

use serde::{Deserialize, Serialize};

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub uid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn create_user(name: String, password: String) -> MyResult<Tokens> {
    let mut body = serde_json::Map::new();
    body.insert("name".into(), name.into());
    body.insert("password".into(), password.into());

    let res = client()
        .post(format!("{}/user", CONFIG.auth_origin))
        .json(&body)
        .send()
        .await
        .map_err(|_| MyError::new_bad_request())?;

    let tokens = res
        .json::<Tokens>()
        .await
        .map_err(|_| MyError::new_bad_request())?;

    Ok(tokens)
}

pub async fn get_user(access_token: String) -> MyResult<Auth> {
    let res = client()
        .get(format!("{}/user", CONFIG.auth_origin))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|_| MyError::new_unauthorized())?;

    let auth = res
        .json::<Auth>()
        .await
        .map_err(|_| MyError::new_unauthorized())?;

    Ok(auth)
}
