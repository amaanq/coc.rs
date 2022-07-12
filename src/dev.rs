use reqwest::{Error, RequestBuilder, Response};
use serde::{ Deserialize, Serialize };
use crate::api::Client;

// manage a session
pub const BASE_DEV_URL: &str = "https://developer.clashofclans.com/api";
const IP_URL: &str = "https://api.ipify.org";

pub async fn login(email: String, password: String) -> reqwest::Client {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    client.post(format!("{}/login", BASE_DEV_URL))
        .json::<Creds>(&Creds {
            email,
            password,
        })
        .send()
        .await.expect("TODO: panic message");
    client
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creds {
    pub email: String,
    pub password: String,
}

async fn get_ip() -> Result<String, reqwest::Error> {
    let res = reqwest::Client::new().get(IP_URL).send().await?;
    let ip = res.text().await?;
    Ok(ip)
}

