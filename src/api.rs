use regex::Regex;
use serde::{Deserialize, Serialize};

// extern crate reqwest;

use crate::{
    credentials::Credentials,
    dev::{self, CLIENT},
    models::*,
};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    RequestBuilder, Url,
};
use serde::de::DeserializeOwned;

use std::sync::{Arc, Mutex};

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct Client {
    credentials: Arc<Mutex<Credentials>>,
    ready: Arc<Mutex<bool>>,
    accounts: Arc<Mutex<Vec<dev::APIAccount>>>,
    index: Arc<Mutex<dev::Index>>,

    ip_address: Arc<Mutex<String>>,

    pub(crate) is_cos_logged_in: Arc<Mutex<bool>>,
}

#[derive(Debug)]
pub enum APIError {
    /// API hasn't been initialized yet (logging in + making keys).
    ClientNotReady,
    /// Failed to query the current ip address.
    FailedGetIP(String),
    /// Failed to login to an account, either due to invalid credentials or a server error.
    LoginFailed(String),
    /// Reqwest error
    RequestFailed(reqwest::Error),
    /// Status code of 400
    BadParameters,
    /// Status code of 403
    AccessDenied,
    /// Status code of 404
    NotFound,
    /// Status code of 429
    RequestThrottled,
    /// Status code of 500
    UnknownError,
    /// Status code of 503
    InMaintenance,
    /// All other cases (edge cases/unknown status codes)
    BadResponse(String, reqwest::StatusCode),
    /// From malformed cursors or using invalid leagues
    InvalidParameters(String),

    EventFailure(String),
}

impl Client {
    const IP_URL: &'static str = "https://api.ipify.org";

    const BASE_URL: &'static str = "https://api.clashofclans.com/v1";
    // const CLAN_ENDPOINT: &str = "/clans/{}";
    // const CLAN_WARLOG_ENDPOINT: &str = "/clans/{}/warlog";
    // const PLAYER_ENDPOINT: &str = "/players/{}";
    // const LEAGUE_ENDPOINT: &str = "/leagues";
    // const WAR_LEAGUE_ENDPOINT: &str = "/warleagues";
    // const LOCATION_ENDPOINT: &str = "/locations";
    const GOLDPASS_ENDPOINT: &'static str = "/goldpass/seasons/current";
    // const LABEL_ENDPOINT: &str = "/labels";

    pub async fn new(credentials: Credentials) -> Result<Self, APIError> {
        let client = Self {
            credentials: Arc::new(Mutex::new(credentials)),
            ready: Arc::new(Mutex::new(false)),
            accounts: Arc::new(Mutex::new(vec![])),
            index: Arc::new(Mutex::new(dev::Index::default())),
            ip_address: Arc::new(Mutex::new(String::new())),

            is_cos_logged_in: Arc::new(Mutex::new(false)),
        };

        client.init().await?;
        *client.ready.lock().unwrap() = true;
        Ok(client)
    }

    pub async fn load(&self, credentials: Credentials) -> Result<(), APIError> {
        *self.credentials.lock().unwrap() = credentials;
        *self.ready.lock().unwrap() = false;
        self.init().await?;
        *self.ready.lock().unwrap() = true;
        Ok(())
    }

    async fn get_ip() -> Result<String, APIError> {
        let res = CLIENT.get(Self::IP_URL).send().await;
        let ip = match res {
            Ok(res) => res.text().await.unwrap(),
            Err(err) => {
                return Err(APIError::FailedGetIP(format!(
                    "client.get_ip(): `{}`",
                    err.to_string(),
                )))
            }
        };
        Ok(ip)
    }

    async fn init(&self) -> Result<(), APIError> {
        let ip = Client::get_ip().await?;
        self.ip_address.lock().unwrap().push_str(&ip);

        for credential in self.credentials.lock().unwrap().0.iter() {
            let account = dev::APIAccount::login(credential, ip.clone()).await;
            self.accounts.lock().unwrap().push(account?);
        }
        Ok(())
    }

    pub async fn print_keys(&self) {
        for account in self.accounts.lock().unwrap().iter() {
            for key in account.keys.keys.iter() {
                println!("{key}");
            }
        }
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // HTTP Methods
    //_______________________________________________________________________
    pub(crate) fn get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::RequestBuilder, APIError> {
        if !*self.ready.lock().unwrap() {
            return Err(APIError::ClientNotReady);
        }
        Ok(CLIENT.get(url).bearer_auth(&self.cycle()))
    }

    pub(crate) fn post<U: reqwest::IntoUrl, T: Into<reqwest::Body>>(
        &self,
        url: U,
        body: T,
    ) -> Result<reqwest::RequestBuilder, APIError> {
        if !*self.ready.lock().unwrap() {
            return Err(APIError::ClientNotReady);
        }
        Ok(CLIENT.post(url).bearer_auth(&self.cycle()).body(body))
    }

    /// To allow usage without a client being ready
    pub(crate) fn cos_get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::RequestBuilder, APIError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authority",
            HeaderValue::from_str("api.clashofstats.com").unwrap(),
        );
        headers.insert("method", HeaderValue::from_str("GET").unwrap());
        headers.insert("scheme", HeaderValue::from_str("https").unwrap());
        headers.insert(
            "accept",
            HeaderValue::from_str("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9").unwrap(),
        );
        headers.insert(
            "accept-language",
            HeaderValue::from_str("en-US,en;q=0.9,zh-CN;q=0.8,z;q=0.7").unwrap(),
        );
        headers.insert(
            "sec-ch-ua",
            HeaderValue::from_str(
                "\"Not/A)Brand\";v=\"99\", \"Google Chrome\";v=\"103\", \"Chromium\";v=\"103\"",
            )
            .unwrap(),
        );
        headers.insert(
            "sec-ch-ua-platform",
            HeaderValue::from_str("\"Windows\"").unwrap(),
        );
        headers.insert(
            "upgrade-insecure-requests",
            HeaderValue::from_str("1").unwrap(),
        );
        headers.insert(
            "user-agent",
            HeaderValue::from_str("Mozilla/5.0 (X11; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36").unwrap(),
        );

        Ok(CLIENT.get(url).headers(headers))
    }

    /// To allow usage without a client being ready
    pub(crate) fn cos_post<U: reqwest::IntoUrl, T: Into<reqwest::Body>>(
        &self,
        url: U,
        body: T,
    ) -> Result<reqwest::RequestBuilder, APIError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authority",
            HeaderValue::from_str("api.clashofstats.com").unwrap(),
        );
        headers.insert("method", HeaderValue::from_str("POST").unwrap());
        headers.insert("scheme", HeaderValue::from_str("https").unwrap());
        headers.insert(
            "accept",
            HeaderValue::from_str("application/json, text/plain, */*").unwrap(),
        );
        headers.insert(
            "accept-encoding",
            HeaderValue::from_str("gzip, deflate, br").unwrap(),
        );
        headers.insert(
            "accept-language",
            HeaderValue::from_str("en-US,en;q=0.9,zh-CN;q=0.8,z;q=0.7").unwrap(),
        );
        headers.insert(
            "content-type",
            HeaderValue::from_str("application/json;charset=UTF-8").unwrap(),
        );
        headers.insert(
            "sec-ch-ua",
            HeaderValue::from_str(
                "\"Not/A)Brand\";v=\"99\", \"Google Chrome\";v=\"103\", \"Chromium\";v=\"103\"",
            )
            .unwrap(),
        );
        headers.insert(
            "sec-ch-ua-platform",
            HeaderValue::from_str("\"Windows\"").unwrap(),
        );
        headers.insert(
            "upgrade-insecure-requests",
            HeaderValue::from_str("1").unwrap(),
        );
        headers.insert(
            "user-agent",
            HeaderValue::from_str("Mozilla/5.0 (X11; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36").unwrap(),
        );

        Ok(CLIENT.post(url).body(body).headers(headers))
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Clan Methods
    //_______________________________________________________________________
    pub async fn get_clan_warlog(
        &self,
        mut clan_tag: String,
    ) -> Result<APIResponse<war_log::WarLog>, APIError> {
        clan_tag = self.fix_tag(clan_tag);
        let url = format!(
            "{}/clans/{}/warlog",
            Self::BASE_URL,
            urlencoding::encode(&clan_tag)
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_clans(
        &self,
        options: clan_search::ClanSearchOptions,
    ) -> Result<APIResponse<clan::Clan>, APIError> {
        let url =
            Url::parse_with_params(&format!("{}/clans", Self::BASE_URL), options.items).unwrap();
        self.parse_json(self.get(url.to_string())).await
    }

    pub async fn get_current_war(&self, mut clan_tag: String) -> Result<war::War, APIError> {
        clan_tag = self.fix_tag(clan_tag);
        let url = format!(
            "{}/clans/{}/currentwar",
            Self::BASE_URL,
            urlencoding::encode(&clan_tag)
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_clan(&self, mut clan_tag: String) -> Result<clan::Clan, APIError> {
        clan_tag = self.fix_tag(clan_tag);
        let url = format!(
            "{}/clans/{}",
            Self::BASE_URL,
            urlencoding::encode(&clan_tag)
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_clan_members(
        &self,
        mut clan_tag: String,
    ) -> Result<APIResponse<clan::ClanMember>, APIError> {
        clan_tag = self.fix_tag(clan_tag);
        let url = format!(
            "{}/clans/{}/members",
            Self::BASE_URL,
            urlencoding::encode(&clan_tag)
        );
        self.parse_json(self.get(url)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Player Methods
    //_______________________________________________________________________
    pub async fn get_player(&self, mut player_tag: String) -> Result<player::Player, APIError> {
        player_tag = self.fix_tag(player_tag);
        let url = format!(
            "{}/players/{}",
            Self::BASE_URL,
            urlencoding::encode(&player_tag)
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn verify_player_token(
        &self,
        mut player_tag: String,
        token: String,
    ) -> Result<player::PlayerToken, APIError> {
        player_tag = self.fix_tag(player_tag);
        let url = format!(
            "{}/players/{}/verifytoken",
            Self::BASE_URL,
            urlencoding::encode(&player_tag)
        );
        let token = format!("{{\"token\":\"{}\"}}", token);
        self.parse_json(self.post(url, token)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // League Methods
    //_______________________________________________________________________
    pub async fn get_leagues(&self) -> Result<APIResponse<leagues::League>, APIError> {
        let url = format!("{}/leagues", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    // /leagues/{leagueId}/seasons/{seasonId}
    pub async fn get_league_season_rankings(
        &self,
        league_id: leagues::LeagueKind,
        mut season_id: season::Season,
        paging: paging::Paging,
    ) -> Result<APIResponse<rankings::PlayerRanking>, APIError> {
        if league_id != leagues::LeagueKind::LegendLeague {
            return Err(APIError::InvalidParameters(
                "This league does not have seasons, only League::LegendLeague has seasons"
                    .to_string(),
            ));
        }
        let mut url = format!(
            "{}/leagues/{}/seasons/{}",
            Self::BASE_URL,
            league_id as i32,
            season_id.to_string()
        );
        if paging.is_some() {
            url = Url::parse_with_params(&url, paging.to_vec())
                .unwrap()
                .to_string();
        }
        self.parse_json(self.get(url)).await
    }

    pub async fn get_league(
        &self,
        league_id: leagues::LeagueKind,
    ) -> Result<leagues::League, APIError> {
        let url = format!("{}/leagues/{}", Self::BASE_URL, league_id as i32);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_league_seasons(
        &self,
        league_id: leagues::LeagueKind,
    ) -> Result<APIResponse<season::Season>, APIError> {
        if league_id != leagues::LeagueKind::LegendLeague {
            return Err(APIError::InvalidParameters(
                "This league does not have seasons, only League::LegendLeague has seasons"
                    .to_string(),
            ));
        }
        let url = format!("{}/leagues/{}/seasons", Self::BASE_URL, league_id as i32);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_war_league(
        &self,
        war_league: leagues::WarLeagueKind,
    ) -> Result<leagues::WarLeague, APIError> {
        let url = format!("{}/warleagues/{}", Self::BASE_URL, war_league as i32);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_war_leagues(&self) -> Result<APIResponse<leagues::WarLeague>, APIError> {
        let url = format!("{}/warleagues", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Location Methods
    //_______________________________________________________________________

    pub async fn get_clan_rankings(
        &self,
        location: location::Local,
    ) -> Result<APIResponse<rankings::ClanRanking>, APIError> {
        let url = format!(
            "{}/locations/{}/rankings/clans",
            Self::BASE_URL,
            location as i32,
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_player_rankings(
        &self,
        location: location::Local,
    ) -> Result<APIResponse<rankings::PlayerRanking>, APIError> {
        let url = format!(
            "{}/locations/{}/rankings/players",
            Self::BASE_URL,
            location as i32,
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_versus_clan_rankings(
        &self,
        location: location::Local,
    ) -> Result<APIResponse<rankings::ClanRanking>, APIError> {
        let url = format!(
            "{}/locations/{}/rankings/clans-versus",
            Self::BASE_URL,
            location as i32,
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_versus_player_rankings(
        &self,
        location: location::Local,
    ) -> Result<APIResponse<rankings::PlayerVersusRanking>, APIError> {
        let url = format!(
            "{}/locations/{}/rankings/players-versus",
            Self::BASE_URL,
            location as i32,
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_locations(&self) -> Result<APIResponse<location::Location>, APIError> {
        let url = format!("{}/locations", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_location(
        &self,
        location: location::Local,
    ) -> Result<location::Location, APIError> {
        let url = format!("{}/locations/{}", Self::BASE_URL, location as i32);
        self.parse_json(self.get(url)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Gold Pass Method
    //_______________________________________________________________________
    pub async fn get_goldpass(&self) -> Result<gold_pass::GoldPass, APIError> {
        let url = format!("{}{}", Self::BASE_URL, Self::GOLDPASS_ENDPOINT);
        self.parse_json(self.get(url)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Label Methods
    //_______________________________________________________________________
    pub async fn get_player_labels(&self) -> Result<APIResponse<labels::PlayerLabel>, APIError> {
        let url = format!("{}/labels/players", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_clan_labels(&self) -> Result<APIResponse<labels::ClanLabel>, APIError> {
        let url = format!("{}/labels/clans", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    #[allow(dead_code)]
    fn is_valid_tag(&self, tag: String) -> bool {
        Regex::new("^#[PYLQGRJCUV0289]+$")
            .unwrap()
            .is_match(&tag.to_uppercase().replace("O", "0"))
    }

    pub fn fix_tag(&self, tag: String) -> String {
        let re = Regex::new("[^A-Z0-9]+").unwrap();
        "#".to_owned() + &re.replace_all(&tag.to_uppercase(), "").replace("O", "0")
    }

    /// Runs the future that implements `Send` and parses the reqwest response into a `APIResponse`.
    pub(crate) async fn parse_json<T: DeserializeOwned>(
        &self,
        rb: Result<RequestBuilder, APIError>,
    ) -> Result<T, APIError> {
        match rb {
            Ok(rb) => {
                match rb.send().await {
                    Ok(resp) => {
                        match resp.status() {
                            reqwest::StatusCode::OK => {
                                let text = resp.text().await.unwrap();
                                Ok(serde_json::from_str(&text).expect(&format!(
                                    "Failure parsing json (please file a bug on the GitHub): {}",
                                    text
                                )))
                            }
                            // 400
                            reqwest::StatusCode::BAD_REQUEST => Err(APIError::BadParameters),
                            // 403
                            reqwest::StatusCode::FORBIDDEN => Err(APIError::AccessDenied),
                            // 404
                            reqwest::StatusCode::NOT_FOUND => Err(APIError::NotFound),
                            // 429
                            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                                Err(APIError::RequestThrottled)
                            }
                            // 500
                            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                                Err(APIError::UnknownError)
                            }
                            // 503
                            reqwest::StatusCode::SERVICE_UNAVAILABLE => {
                                Err(APIError::InMaintenance)
                            }
                            // edge cases
                            _ => {
                                let status = resp.status();
                                Err(APIError::BadResponse(resp.text().await.unwrap(), status))
                            }
                        }
                    }
                    Err(e) => Err(APIError::RequestFailed(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn cycle(&self) -> String {
        // increment key_token_index, unless it would be larger than the account's token size (10), then reset to 0 and increment key_account_index
        let mut unlocked = self.index.lock().unwrap();

        let mut key_token_index = unlocked.key_token_index;
        let mut key_account_index = unlocked.key_account_index;

        let unlocked_accounts = self.accounts.lock().unwrap();
        if key_token_index
            == (unlocked_accounts[key_account_index as usize]
                .keys
                .keys
                .len()
                - 1) as i8
        {
            key_token_index = 0;
            if key_account_index == (unlocked_accounts.len() - 1) as i8 {
                key_account_index = 0;
            } else {
                key_account_index += 1;
            }
        } else {
            key_token_index += 1;
        }

        unlocked.key_token_index = key_token_index;
        unlocked.key_account_index = key_account_index;

        let token = unlocked_accounts
            .get(key_account_index as usize)
            .unwrap()
            .keys
            .keys
            .get(key_token_index as usize)
            .unwrap()
            .clone();
        token.key
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse<T> {
    pub items: Vec<T>,
    pub paging: paging::Paging,
}
