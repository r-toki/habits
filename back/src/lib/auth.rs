use crate::lib::{
    config::CONFIG,
    my_error::{MyError, MyResult},
};

use serde::{Deserialize, Serialize};

fn client() -> reqwest::Client {
    reqwest::Client::new()
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
