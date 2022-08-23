use regex::Regex;
use serde::{Deserialize, Serialize};

extern crate reqwest;

use crate::credentials::Credentials;
use crate::models::clan::{Clan, ClanMember};
use crate::models::current_war::War;
use crate::models::gold_pass::GoldPass;
use crate::models::player::{Player, PlayerToken};

use crate::models::war_log::WarLog;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use crate::dev::{self, get_ip, APIAccount, CLIENT};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Client {
    credentials: Credentials,
    ready: bool,

    accounts: Arc<Mutex<Vec<dev::APIAccount>>>,
    index: Arc<Mutex<dev::Index>>,

    ip_address: Arc<Mutex<String>>,
}

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    Api(reqwest::StatusCode),
}

// lazy_static! {
//     static ref TOKEN_LIST: Mutex<Vec<String>> = Mutex::new(vec![]);
//     static ref s_Client: reqwest::Client = reqwest::Client::new();
// }

const BASE_URL: &str = "https://api.clashofclans.com/v1";

impl Client {
    pub async fn new(credentials: Credentials) -> Self {
        let mut client = Self {
            credentials,
            ready: false,
            accounts: Arc::new(Mutex::new(vec![])),
            index: Arc::new(Mutex::new(dev::Index::default())),
            ip_address: Arc::new(Mutex::new(String::new())),
        };

        client.init().await;
        client.ready = true;
        client
    }

    async fn init(&self) {
        let ip = get_ip().await.unwrap();
        self.ip_address.lock().unwrap().push_str(&ip);

        for credential in self.credentials.0.iter() {
            let account = APIAccount::login(credential, ip.clone()).await;
            self.accounts.lock().unwrap().push(account);
        }
    }

    fn get(&self, url: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let res = CLIENT.get(url).bearer_auth(&self.cycle());
        Ok(res)
    }

    fn post(&self, url: String, body: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let res = CLIENT.post(url).bearer_auth(&self.cycle()).body(body);
        Ok(res)
    }

    //                                                            //
    //                                                            //
    // --------------------------ENDPOINTS----------------------- //
    //                                                            //
    //                                                            //
    pub async fn get_clan(&self, tag: String) -> Result<Clan, ApiError> {
        let url = format!("{}/clans/{}", BASE_URL, self.format_tag(tag));
        self.parse_json::<Clan>(self.get(url)).await
    }

    pub async fn get_player(&self, tag: String) -> Result<Player, ApiError> {
        let url = format!("{}/players/{}", BASE_URL, self.format_tag(tag));
        self.parse_json::<Player>(self.get(url)).await
    }

    pub async fn get_current_war(&self, tag: String) -> Result<War, ApiError> {
        let url = format!("{}/clans/{}/currentwar", BASE_URL, self.format_tag(tag));
        self.parse_json::<War>(self.get(url)).await
    }

    pub async fn get_goldpass(&self) -> Result<GoldPass, ApiError> {
        let url = format!("{}/goldpass/seasons/current", BASE_URL);
        self.parse_json::<GoldPass>(self.get(url)).await
    }

    pub async fn get_verified_player(
        &self,
        tag: String,
        token: String,
    ) -> Result<PlayerToken, ApiError> {
        let url = format!("{}/players/{}/verifytoken", BASE_URL, self.format_tag(tag));
        let token = format!("{{\"token\":\"{}\"}}", token);
        self.parse_json::<PlayerToken>(self.post(url, token)).await
    }

    pub async fn get_clan_members(
        &self,
        tag: String,
        config: ConfigForRezponse,
    ) -> Result<Rezponse<ClanMember>, ApiError> {
        let mut url = format!(
            "https://api.clashofclans.com/v1/clans/{}/members",
            self.format_tag(tag)
        );
        url = self.get_cursor_url(url, config);
        self.parse_json::<Rezponse<ClanMember>>(self.get(url)).await
    }

    pub async fn get_clan_warlog(
        &self,
        tag: String,
        config: ConfigForRezponse,
    ) -> Result<Rezponse<WarLog>, ApiError> {
        let mut url = format!(
            "https://api.clashofclans.com/v1/clans/{}/warlog",
            self.format_tag(tag)
        );
        url = self.get_cursor_url(url, config);
        self.parse_json::<Rezponse<WarLog>>(self.get(url)).await
    }

    //                                                            //
    //                                                            //
    // --------------------------END POINTS-----------------------//
    //                                                            //
    //                                                            //

    fn get_cursor_url(&self, mut url: String, config: ConfigForRezponse) -> String {
        match config.limit {
            Some(s) => {
                url = format!("{}?limit={}", url, s);
                match config.time {
                    None => url,
                    Some(s1) => match s1 {
                        Time::After(a) => {
                            format!("{}&after={}", url, a)
                        }
                        Time::Before(b) => {
                            format!("{}&before={}", url, b)
                        }
                    },
                }
            }
            None => match config.time {
                None => url,
                Some(s) => match s {
                    Time::After(a) => {
                        format!("{}?after={}", url, a)
                    }
                    Time::Before(b) => {
                        format!("{}?before={}", url, b)
                    }
                },
            },
        }
    }

    fn format_tag(&self, tag: String) -> String {
        return if tag[0..1].contains("#") {
            tag.replace("#", "%23")
        } else {
            format!("%23{}", tag)
        };
    }

    #[allow(dead_code)]
    fn is_valid_tag(&self, tag: String) -> bool {
        Regex::new("^#[PYLQGRJCUV0289]+$")
            .unwrap()
            .is_match(tag.to_uppercase().replace("O", "0").as_str())
    }

    #[allow(dead_code)]
    fn fix_tag(&self, tag: String) -> String {
        let re = Regex::new("[^A-Z0-9]+").unwrap();
        "#".to_owned()
            + &re
                .replace_all(tag.to_uppercase().as_str(), "")
                .replace("O", "0")
    }

    async fn parse_json<T: DeserializeOwned>(
        &self,
        rb: Result<RequestBuilder, reqwest::Error>,
    ) -> Result<T, ApiError> {
        match rb {
            Ok(rb) => match rb.send().await {
                Ok(res) => match res.status() {
                    reqwest::StatusCode::OK => {
                        let t: String = res
                            .json()
                            .await
                            .expect("Unexpected json response from the API, cannot parse json");
                        Ok(serde_json::from_str(t.as_str()).unwrap())
                    }
                    _ => Err(ApiError::Api(res.status())),
                },
                Err(e) => Err(ApiError::Request(e)),
            },
            Err(e) => return Err(ApiError::Request(e)),
        }
    }

    fn cycle(&self) -> String {
        // TOKEN_LIST.lock().unwrap().rotate_left(1);
        // TOKEN_LIST
        //     .lock()
        //     .unwrap()
        //     .get(0)
        //     .unwrap()
        //     .to_string()
        //     .clone()

        // increment key_token_index, unless it would be larger than the account's token size (10), then reset to 0 and increment key_account_index
        let unlocked = self.index.lock().unwrap();
        let mut key_token_index = unlocked.key_token_index;
        let mut key_account_index = unlocked.key_account_index;
        if key_token_index
            == (self.accounts.lock().unwrap()[key_account_index as usize]
                .keys
                .keys
                .len()
                - 1) as i8
        {
            key_token_index = 0;
            if key_account_index == (self.accounts.lock().unwrap().len() - 1) as i8 {
                key_account_index = 0;
            } else {
                key_account_index += 1;
            }
        } else {
            key_token_index += 1;
        }

        self.index.lock().unwrap().key_token_index = key_token_index;
        self.index.lock().unwrap().key_account_index = key_account_index;

        let token = self
            .accounts
            .lock()
            .unwrap()
            .get(key_account_index as usize)
            .unwrap()
            .keys
            .keys
            .get(key_token_index as usize)
            .unwrap()
            .clone();

        token.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paging {
    #[serde(rename = "cursors")]
    cursor: Cursor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cursor {
    before: Option<String>,
    after: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rezponse<T> {
    #[serde(rename = "items")]
    items: Vec<T>,
    #[serde(rename = "paging")]
    paging: Paging,
}

#[derive(Debug)]
pub struct ConfigForRezponse {
    pub limit: Option<u32>,
    pub time: Option<Time>,
}

#[derive(Debug)]
pub enum Time {
    After(String),
    Before(String),
}
