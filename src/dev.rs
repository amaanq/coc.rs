use crate::credentials::Credential;
use lazy_static::lazy_static;
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
pub struct Keys {
    pub status: Status,
    #[serde(rename = "sessionExpiresInSeconds")]
    pub session_expires_in_seconds: i32,
    pub keys: Vec<Key>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Auth {
    pub uid: String,
    pub token: String,
    pub ua: Option<String>,
    pub ip: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
    keys: Vec<Key>,
}

// manage a session
pub const BASE_DEV_URL: &str = "https://developer.clashofclans.com";
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
        let _login_response = CLIENT
            .post(format!("{}{}", BASE_DEV_URL, Self::LOGIN_ENDPOINT))
            .header("Content-Type", "application/json")
            .json::<Credential>(credential)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("LOGIN RESPONSE: {}", _login_response);

        let login_response =
            serde_json::from_str::<LoginResponse>(_login_response.as_str()).unwrap();
        // .json::<LoginResponse>()
        // .await
        // .unwrap();

        let mut account = Self {
            credential: credential.clone(),
            response: login_response,
            keys: Keys::default(),
        };

        account.get_keys().await;

        // if account.keys.keys.len() != 10 {
        //     println!("CREATING {} KEYS", 10 - account.keys.keys.len());
        //     for _ in 0..(10 - account.keys.keys.len()) {
        //         account.create_key(ip.clone()).await;
        //     }
        // }

        // account.update_all_keys(ip).await;
        println!("CREATING A KEY");
        account.create_key(ip).await;
        println!("CREATED A KEY");

        println!("LOGGED INTO {}", credential.email());
        account
    }

    pub async fn get_keys(&mut self) {
        // set self.keys to response body
        self.keys = CLIENT
            .post(format!("{}{}", BASE_DEV_URL, APIAccount::KEY_LIST_ENDPOINT))
            .send()
            .await
            .unwrap()
            .json::<Keys>()
            .await
            .unwrap();
    }

    pub async fn update_all_keys(&mut self, ip: String) {
        let cloned_keys = self.keys.clone();
        let bad_keys = cloned_keys
            .keys
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
            .post({
                let url = format!("{}{}", BASE_DEV_URL, APIAccount::KEY_CREATE_ENDPOINT);
                println!("CREATING KEY AT {}", url);
                url
            })
            // .json::<Key>(&Key {
            //     name: "coc.rs".to_string(),
            //     description: "Created on %s by coc.rs".to_string(),
            //     cidr_ranges: vec![ip.to_string()],
            //     scopes: vec![Scope::Clash],
            // })
            // body as string
            // sample json {"name":"coc-rs","description":"Created on 2022-08-24T06:34:28Z","cidrRanges":["1.1.1.1"],"scopes":["clash"]}
            .body({
                let body = format!(
                    r#"{{"name":"hi","description":"hi","cidrRanges":["{}"]}}"#,
                    //chrono::Utc::now().to_rfc3339(),
                    ip
                );
                println!("{}", body);
                body
            })
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("KEY CREATE: {}", key);

        // asynchronously call self.get_keys()
        self.get_keys().await;

        serde_json::from_str(&key).unwrap()
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
    #[allow(dead_code)]
    pub fn get_key(&self, index: usize) -> Key {
        self.keys[index].clone()
    }

    #[allow(dead_code)]
    pub fn remove_invalid_keys(&mut self, ip: &str) {
        // get ip
        self.keys
            .retain(|key| key.cidr_ranges.iter().any(|cidr| cidr.contains(&ip)));
    }
}
