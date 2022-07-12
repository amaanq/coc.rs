use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt::format;
use std::ops::Index;
use std::sync::Mutex;

extern crate reqwest;

use crate::models::clan::{Clan, ClanMember};
use crate::models::current_war::War;
use crate::models::gold_pass::GoldPass;
use crate::models::player::{Player, PlayerToken};

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use crate::models::war_log::WarLog;

#[derive(Debug)]
pub struct Client {
    token: String,
}

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    Api(reqwest::StatusCode),
}

static TOKEN_LIST: Vec<String> = Vec::new();

const BASE_URL: &str = "https://api.clashofclans.com/v1";

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            token,
        }
    }

    pub fn login(username: String, password: String) -> Self {
        Self {
            token: username,
        }
    }

    fn get(&self, url: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let res = reqwest::Client::new()
            .get(url)
            .bearer_auth(&self.token);
        Ok(res)
    }

    fn post(&self, url: String, body: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let res = reqwest::Client::new()
            .post(url)
            .bearer_auth(&self.token)
            .body(body);
        Ok(res)
    }

    ///                                                            ///
    ///                                                            ///
    /// --------------------------END POINTS-----------------------///
    ///                                                            ///
    ///                                                            ///
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

    pub async fn get_goldpass(&self, tag: String) -> Result<GoldPass, ApiError> {
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

    pub async fn get_clan_members(&self, tag: String, config: ConfigForRezponse) -> Result<Rezponse<ClanMember>, ApiError> {
        let mut url = format!("https://api.clashofclans.com/v1/clans/{}/members", self.format_tag(tag));
        url = self.get_cursor_url(url, config);
        self.parse_json::<Rezponse<ClanMember>>(self.get(url)).await
    }

    pub async fn get_clan_warlog(&self, tag: String, config: ConfigForRezponse) -> Result<Rezponse<WarLog>, ApiError>{
        let mut url = format!("https://api.clashofclans.com/v1/clans/{}/warlog", self.format_tag(tag));
        url = self.get_cursor_url(url, config);
        self.parse_json::<Rezponse<WarLog>>(self.get(url)).await
    }

    ///                                                            ///
    ///                                                            ///
    /// --------------------------END POINTS-----------------------///
    ///                                                            ///
    ///                                                            ///

    fn get_cursor_url(&self, mut url: String, config: ConfigForRezponse) -> String {
        match config.limit {
            Some(s) => {
                url = format!("{}?limit={}",url,  s);
                match config.time {
                    None => {
                        url
                    }
                    Some(s1) => {
                        match s1 {
                            Time::After(a) => { format!("{}&after={}",url, a)}
                            Time::Before(b) => { format!("{}&before={}",url, b)}
                        }
                    }
                }
            }
            None => {
                match config.time{
                    None => {
                        url
                    }
                    Some(s) => {
                        match s {
                            Time::After(a) => { format!("{}?after={}",url, a)}
                            Time::Before(b) => { format!("{}?before={}",url, b)}
                        }
                    }
                }
            }

        }
    }


    fn format_tag(&self, tag: String) -> String {
        return if tag[0..1].eq_ignore_ascii_case("#") {
            tag.replace("#", "%23")
        } else {
            format!("%23{}", tag)
        };
    }

    fn is_valid_tag(&self, tag: String) -> bool {
        Regex::new("^#[PYLQGRJCUV0289]+$").unwrap()
            .is_match(
                tag.to_uppercase()
                    .replace("O", "0")
                    .as_str()
            )
    }

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
                        let t = res
                            .text()
                            .await
                            .expect("Unexpected json response from the API, cannot parse json");
                        //println!("{}", &t);
                        Ok(serde_json::from_str(t.as_str()).unwrap())
                    },
                    _ => Err(ApiError::Api(res.status())),
                },
                Err(e) => Err(ApiError::Request(e)),
            },
            Err(e) => return Err(ApiError::Request(e)),
        }
    }
    //
    // fn cycle() {
    //     TOKEN_LIST.lock().unwrap().rotate_left(1);
    // }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paging {
    #[serde(rename= "cursors")]
    cursor: Cursor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cursor {
    before: Option<String>,
    after: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rezponse<T> {
    #[serde(rename="items")]
    items: Vec<T>,
    #[serde(rename="paging")]
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