use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::error::Error;

extern crate reqwest;

use crate::entites::player::Player;
use crate::entites::clan::Clan;
use crate::entites::current_war::War;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Client {
    token: String,
}

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    Api(reqwest::StatusCode),
}

const BASE_URL: &str = "https://api.clashofclans.com/v1";

impl Client {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    fn get(&self, url: String) -> Result<reqwest::blocking::RequestBuilder, reqwest::Error> {
        let string = format!("Bearer {}", &self.token);
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&string).unwrap());
        let res = reqwest::blocking::Client::new().get(url).headers(headers);
        Ok(res)
    }

    pub fn get_clan(&self, tag: String) -> Result<Clan, ApiError> {
        let url = format!("{}/clans/{}", BASE_URL, self.format_tag(tag));
        self.parse_json::<Clan>(url)
    }

    pub fn get_player(&self, tag: String) -> Result<Player, ApiError> {
        let url = format!("{}/players/{}", BASE_URL, self.format_tag(tag));
        self.parse_json::<Player>(url)
    }
    pub fn get_current_war(&self, tag: String) -> Result<War, ApiError> {
        let url = format!("{}/clans/{}/currentwar", BASE_URL, self.format_tag(tag));
        self.parse_json::<War>(url)
    }
    //It should return a String of "%23+tag"
    fn format_tag(&self, tag: String) -> String {
        return if tag[0..1].eq_ignore_ascii_case("#") {
            tag.replace("#", "%23")
        } else {
            format!("%23{}", tag)
        };
    }

    fn parse_json<T: DeserializeOwned>(&self, url: String) -> Result<T, ApiError> {
        let rb = self.get(url);

        match rb {
            Ok(rb) => match rb.send() {
                Ok(res) => match res.status() {
                    reqwest::StatusCode::OK => Ok(res.json().expect("Unexpected json response from the API, cannot parse json")),
                    _ => Err(ApiError::Api(res.status())),
                },
                Err(e) => Err(ApiError::Request(e)),
            },
            Err(e) => return Err(ApiError::Request(e)),
        }
    }
}
