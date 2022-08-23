use std::{default, sync::Arc};

use crate::{api::Client, credentials::Credential};
use lazy_static::lazy_static;
use reqwest::{Error, RequestBuilder, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Index {
    pub key_account_index: i8,
    pub key_token_index: i8,
}

#[derive(Debug, Default)]
pub struct APIAccount {
    pub credential: Credential,
    pub response: LoginResponse,
    pub keys: Keys,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Keys(pub Vec<Key>);

// type LoginResponse struct {
// 	Status                  Status    `json:"status"`
// 	SessionExpiresInSeconds int       `json:"sessionExpiresInSeconds"`
// 	Auth                    Auth      `json:"auth"`
// 	Developer               Developer `json:"developer"`
// 	TemporaryAPIToken       string    `json:"temporaryAPIToken"`
// 	SwaggerURL              string    `json:"swaggerUrl"`
// }

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: Status,
    pub session_expires_in_seconds: i32,
    pub auth: Auth,
    pub developer: Developer,
    pub temporary_api_token: String,
    pub swagger_url: String,
}

// type Auth struct {
// 	Uid   string `json:"uid"`
// 	Token string `json:"token"`
// 	Ua    any    `json:"ua"`
// 	IP    any    `json:"ip"`
// }
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Auth {
    pub uid: String,
    pub token: String,
    pub ua: String,
    pub ip: String,
}

// type Developer struct {
// 	ID            string `json:"id"`
// 	Name          string `json:"name"`
// 	Game          string `json:"game"`
// 	Email         string `json:"email"`
// 	Tier          string `json:"tier"`
// 	AllowedScopes any    `json:"allowedScopes"`
// 	MaxCidrs      any    `json:"maxCidrs"`
// 	PrevLoginTs   string `json:"prevLoginTs"`
// 	PrevLoginIP   string `json:"prevLoginIp"`
// 	PrevLoginUa   string `json:"prevLoginUa"`
// }
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Developer {
    pub id: String,
    pub name: String,
    pub game: String,
    pub email: String,
    pub tier: String,
    pub allowed_scopes: String,
    pub max_cidrs: String,
    pub prev_login_ts: String,
    pub prev_login_ip: String,
    pub prev_login_ua: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Key {
    pub id: String,
    #[serde(rename = "developerId")]
    pub developer_id: String,
    pub tier: String,
    pub name: String,
    pub description: String,
    pub origins: Option<String>,
    pub scopes: Vec<Scope>,
    #[serde(rename = "cidrRanges")]
    pub cidr_ranges: Vec<String>,
    #[serde(rename = "validUntil")]
    pub valid_until: Option<String>,
    pub key: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "clash")]
    #[default]
    Clash,
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
    keys: Vec<Key>,
}

// manage a session
pub const BASE_DEV_URL: &str = "https://developer.clashofclans.com/api";
const IP_URL: &str = "https://api.ipify.org";

lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
}

pub async fn get_ip() -> Result<String, reqwest::Error> {
    Ok(CLIENT.get(IP_URL).send().await?.text().await?)
}

// pub async fn get_keys(username: String, password: String) -> Keys {
//     /*let client =*/
//     login(username, password).await;
//     CLIENT
//         .post(format!("{}/apikey/list", BASE_DEV_URL))
//         .send()
//         .await
//         .unwrap()
//         .json()
//         .await
//         .unwrap()
// }

impl APIAccount {
    const LOGIN_ENDPOINT: &str = "/api/login";
    const KEY_LIST_ENDPOINT: &str = "/api/apikey/list";
    const KEY_CREATE_ENDPOINT: &str = "/api/apikey/create";
    const KEY_REVOKE_ENDPOINT: &str = "/api/apikey/revoke";

    // const CLAN_ENDPOINT: &str = "/clans";
    // const PLAYER_ENDPOINT: &str = "/players";
    // const LEAGUE_ENDPOINT: &str = "/leagues";
    // const WAR_LEAGUE_ENDPOINT: &str = "/warleagues";
    // const LOCATION_ENDPOINT: &str = "/locations";
    // const GOLDPASS_ENDPOINT: &str = "/goldpass/seasons/current";
    // const LABEL_ENDPOINT: &str = "/labels";
    pub async fn login(credential: &Credential, ip: String) -> Self {
        let login_response = CLIENT
            .post(format!("{}{}", BASE_DEV_URL, Self::LOGIN_ENDPOINT))
            .json::<Credential>(credential)
            .send()
            .await
            .unwrap()
            .json::<LoginResponse>()
            .await
            .unwrap();

        let mut account = Self {
            credential: credential.clone(),
            response: login_response,
            keys: Keys(vec![]),
        };

        account.get_keys().await;

        if account.keys.0.is_empty() {
            for _ in 0..10 {
                account.create_key(ip.clone()).await;
            }
        }
        account.update_all_keys(ip).await;

        account
    }

    pub async fn get_keys(&mut self) {
        // set self.keys to response body
        self.keys = CLIENT
            .post(format!("{}{}", BASE_DEV_URL, APIAccount::KEY_LIST_ENDPOINT))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
    }

    pub async fn update_all_keys(&mut self, ip: String) {
        let cloned_keys = self.keys.clone();
        let bad_keys = cloned_keys
            .0
            .iter()
            .filter(|key| !key.cidr_ranges.iter().any(|cidr| ip.contains(cidr)))
            .collect::<Vec<_>>();
        // iter once to revoke all keys, then iter again to create new ones
        let len = bad_keys.len();

        for key in bad_keys {
            self.revoke_key(&key.id).await;
        }
        for _ in 0..len {
            self.create_key(ip.clone()).await;
        }
    }

    pub async fn create_key(&mut self, ip: String) -> KeyResponse {
        // description = "Created on %s by coc.rs", time.Now().Format(time.RFC3339)
        // post to KEY_CREATE_ENDPOINT with header application/json and body {"name":"%s","description":"%s", "cidrRanges": ["%s"], "scopes": ["clash"]}, where cidrRanges is self ip
        let key = CLIENT
            .post(format!("{}{}", BASE_DEV_URL, APIAccount::KEY_CREATE_ENDPOINT))
            // .json::<Key>(&Key {
            //     name: "coc.rs".to_string(),
            //     description: "Created on %s by coc.rs".to_string(),
            //     cidr_ranges: vec![ip.to_string()],
            //     scopes: vec![Scope::Clash],
            // })
            // body as string
            .body(format!(
                "{{\"name\":\"coc.rs\",\"description\":\"Created on {} by coc.rs\",\"cidrRanges\":[\"{}\"],\"scopes\":[\"clash\"]}}",
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
                ip
            ))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        // asynchronously call self.get_keys()
        self.get_keys().await;

        key
    }

    pub async fn revoke_key(&mut self, key_id: &str) -> KeyResponse {
        // post to KEY_REVOKE_ENDPOINT with header application/json and body {"id":"%s"}, where id is key_id
        let key = CLIENT
            .post(format!(
                "{}{}",
                BASE_DEV_URL,
                APIAccount::KEY_REVOKE_ENDPOINT
            ))
            // .json::<Key>(&Key {
            //     id: key_id.to_string(),
            // })
            // body as string
            .body(format!("{{\"id\":\"{}\"}}", key_id))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        // asynchronously call self.get_keys()
        self.get_keys().await;

        key
    }
}

impl Keys {
    pub fn get_key(&self, index: usize) -> Key {
        self.0[index].clone()
    }

    pub fn remove_invalid_keys(&mut self, ip: &str) {
        // get ip
        self.0
            .retain(|key| key.cidr_ranges.iter().any(|cidr| cidr.contains(&ip)));
    }
}
