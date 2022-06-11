use std::borrow::Borrow;
use std::error::Error;
use serde::{Serialize, Deserialize};

extern crate reqwest;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use crate::entites::_player::*;

#[derive(Debug)]
pub struct Clinet {
    token: String,
}

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    Api(reqwest::StatusCode),
}

const BASE_URL: &str = "https://api.clashofclans.com/v1";

impl Clinet {
    pub fn new(token: String) -> Self { Self { token } }

    async fn get(&self, url: String) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let string = format!("Bearer {}", &self.token);
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&string).unwrap());
        let res = reqwest::Client::new()
            .get(url)
            .headers(headers);
        Ok(res)
    }
    pub async fn get_clan(){

    }
    pub async fn get_player(&self, tag: String) -> Result<Player, ApiError> {
        let url = format!("{}/players/{}", BASE_URL,  self.format_tag(tag));
        // let player = self.get(url).await?.send().await?.json().await?;
        self.parse_json::<Player>(url).await
    }
    //It should return a String of "%23+tag"
    fn format_tag(&self, tag: String) -> String {
        return if tag[0..1].eq_ignore_ascii_case("#") {
            tag.replace("#", "%23")
        } else {
            format!("%23{}", tag)
        };
    }

    async fn parse_json<T: DeserializeOwned> (&self, url: String) -> Result<T, ApiError> {
        //let url = format!("https://api.clashofclans.com/v1/players/{}", self.format_tag(tag));

        let player = self.get(url).await;
        match player {
            Ok(rb) => {
                match rb.send().await {
                    Ok(res) => {
                        match res.status() {
                            reqwest::StatusCode::OK => {
                                Ok(res.json().await.expect("Unexpected json response from the API, cannot parse json"))
                            },
                            _ => {
                                Err(ApiError::Api(res.status()))
                            }
                        }
                    }
                    Err(e) => Err(ApiError::Request(e)),
                }
            }
            Err(e) => return Err(ApiError::Request(e)),
        }
    }
}