use crate::{credentials::Credential, error::APIError};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Index {
    pub key_account_index: usize,
    pub key_token_index: usize,
}

#[derive(Clone, Debug, Default)]
pub struct APIAccount {
    pub credential: Credential,
    pub response: LoginResponse,
    pub keys: Keys,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Keys {
    pub status: Option<Status>,
    #[serde(rename = "sessionExpiresInSeconds")]
    pub session_expires_in_seconds: i32,
    pub keys: Vec<Key>,
}

impl Keys {
    #[must_use]
    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: Status,
    #[serde(rename = "sessionExpiresInSeconds")]
    pub session_expires_in_seconds: i32,
    pub auth: Auth,
    pub developer: Developer,
    #[serde(rename = "temporaryAPIToken")]
    pub temporary_api_token: String,
    #[serde(rename = "swaggerUrl")]
    pub swagger_url: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Auth {
    pub uid: String,
    pub token: String,
    pub ua: Option<String>,
    pub ip: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Developer {
    pub id: String,
    pub name: String,
    pub game: String,
    pub email: String,
    pub tier: String,
    pub allowed_scopes: Option<String>,
    pub max_cidrs: Option<String>,
    #[serde(rename = "prevLoginTs")]
    pub prev_login_ts: String,
    #[serde(rename = "prevLoginIp")]
    pub prev_login_ip: String,
    #[serde(rename = "prevLoginUa")]
    pub prev_login_ua: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq)]
pub struct Key {
    pub id: String,
    #[serde(rename = "developerId")]
    pub developer_id: String,
    pub tier: String,
    pub name: String,
    pub description: Option<String>,
    pub origins: Option<String>,
    pub scopes: Vec<Scope>,
    #[serde(rename = "cidrRanges")]
    pub cidr_ranges: Vec<String>,
    #[serde(rename = "validUntil")]
    pub valid_until: Option<String>,
    pub key: String,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.description.as_ref().map_or("None", |d| d);
        writeln!(
            f,
            "Key {{ id: {}, name: {}, description: {}, key: {}, cidr_ranges: {} }}",
            self.id,
            self.name,
            desc,
            self.key,
            self.cidr_ranges.join(", ")
        )
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    #[serde(rename = "clash")]
    #[default]
    Clash,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Status {
    code: i32,
    message: String,
    detail: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KeyResponse {
    status: Status,
    #[serde(rename = "sessionExpiresInSeconds")]
    session_expires_in_seconds: i64,
    key: Option<Key>,
}

// manage a session
lazy_static! {
    pub static ref CLIENT: reqwest::Client =
        reqwest::Client::builder().cookie_store(true).build().unwrap();
}

impl APIAccount {
    const BASE_DEV_URL: &'static str = "https://developer.clashofclans.com";
    const LOGIN_ENDPOINT: &'static str = "/api/login";
    const KEY_LIST_ENDPOINT: &'static str = "/api/apikey/list";
    const KEY_CREATE_ENDPOINT: &'static str = "/api/apikey/create";
    const KEY_REVOKE_ENDPOINT: &'static str = "/api/apikey/revoke";

    pub async fn login(credential: Credential, ip: &str) -> Result<Self, APIError> {
        let login_response = CLIENT
            .post(format!("{}{}", Self::BASE_DEV_URL, Self::LOGIN_ENDPOINT))
            .header("Content-Type", "application/json")
            .json::<Credential>(&credential)
            .send()
            .await?
            .json()
            .await?;

        let mut account = Self { credential, response: login_response, keys: Keys::default() };

        #[cfg(feature = "tracing")]
        tracing::debug!("getting keys");
        account.get_keys().await?;

        if account.keys.len() != 10 {
            #[cfg(feature = "tracing")]
            tracing::debug!("creating {} keys", 10 - account.keys.len());
            for _ in 0..(10 - account.keys.len()) {
                account.create_key(ip).await?;
            }
        }

        #[cfg(feature = "tracing")]
        tracing::debug!("updating keys");
        account.update_all_keys(ip).await?;

        #[cfg(feature = "tracing")]
        tracing::debug!("getting keys");
        account.get_keys().await?;

        Ok(account)
    }

    pub async fn re_login(&mut self, ip: &str) -> Result<(), APIError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("re-login");
        let login_response = CLIENT
            .post(format!("{}{}", Self::BASE_DEV_URL, Self::LOGIN_ENDPOINT))
            .header("Content-Type", "application/json")
            .json::<Credential>(&self.credential)
            .send()
            .await?
            .json()
            .await?;

        self.response = login_response;

        #[cfg(feature = "tracing")]
        tracing::debug!("getting keys");
        self.get_keys().await?;

        if self.keys.len() != 10 {
            #[cfg(feature = "tracing")]
            tracing::debug!("creating {} keys", 10 - self.keys.len());
            for _ in 0..(10 - self.keys.len()) {
                self.create_key(ip).await?;
            }
        }

        #[cfg(feature = "tracing")]
        tracing::debug!("updating keys");
        self.update_all_keys(ip).await?;

        #[cfg(feature = "tracing")]
        tracing::debug!("getting keys");
        self.get_keys().await?;

        Ok(())
    }

    pub async fn get_keys(&mut self) -> Result<(), APIError> {
        self.keys = CLIENT
            .post(format!("{}{}", Self::BASE_DEV_URL, Self::KEY_LIST_ENDPOINT))
            .send()
            .await?
            .json::<Keys>()
            .await?;

        Ok(())
    }

    pub async fn update_all_keys(&mut self, ip: &str) -> Result<(), APIError> {
        let cloned_keys = self.keys.clone();
        let bad_keys = cloned_keys
            .keys
            .iter()
            .filter(|key| !key.cidr_ranges.iter().any(|cidr| ip.contains(cidr)))
            .collect::<Vec<_>>();

        let tasks = bad_keys.iter().map(|key| self.revoke_key(&key.id)).collect::<Vec<_>>();
        futures::future::join_all(tasks).await.into_iter().for_each(|maybe_key| match maybe_key {
            Ok(_) => {
                // in revokes, we don't get a key back. we must remove the key ourselves.
                self.keys.keys.retain(|key| !bad_keys.contains(&key));
            }
            #[cfg(feature = "tracing")]
            Err(e) => {
                tracing::warn!(error.message = %format!("{e:?}"))
            }
            #[cfg(not(feature = "tracing"))]
            Err(_) => {}
        });

        let tasks = (0..bad_keys.len()).map(|_| self.create_key(ip));
        futures::future::join_all(tasks).await.into_iter().for_each(|maybe_key| match maybe_key {
            Ok(key_response) => {
                if let Some(key) = key_response.key {
                    #[cfg(feature = "tracing")]
                    tracing::trace!("created key: {}", key);
                    self.keys.keys.push(key);
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::error!(response = ?key_response, "why is key none?");
                }
            }
            #[cfg(feature = "tracing")]
            Err(e) => {
                tracing::warn!(error.message = %format!("{e:?}"))
            }
            #[cfg(not(feature = "tracing"))]
            Err(_) => {}
        });

        Ok(())
    }

    pub async fn create_key(&self, ip: &str) -> Result<KeyResponse, APIError> {
        let key = CLIENT
            .post(format!("{}{}", Self::BASE_DEV_URL, Self::KEY_CREATE_ENDPOINT))
            .header("Content-Type", "application/json")
            .body(format!(
                r#"{{"name":"coc-rs","description":"Created on {} by coc.rs","cidrRanges":["{}"],"scopes":["clash"]}}"#,
                chrono::Utc::now().to_rfc3339(),
                ip
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(key)
    }

    pub async fn revoke_key(&self, key_id: &str) -> Result<KeyResponse, APIError> {
        let key = CLIENT
            .post(format!("{}{}", Self::BASE_DEV_URL, Self::KEY_REVOKE_ENDPOINT))
            .header("Content-Type", "application/json")
            .body(format!("{{\"id\":\"{key_id}\"}}"))
            .send()
            .await?
            .json()
            .await?;

        Ok(key)
    }
}
