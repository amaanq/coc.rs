use regex::Regex;
use serde::{Deserialize, Serialize};

extern crate reqwest;

use crate::{
    credentials::Credentials,
    dev::{self, CLIENT},
    models::*,
};

use reqwest::{RequestBuilder, Url};
use serde::de::DeserializeOwned;

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct Client {
    credentials: Credentials,
    ready: bool,

    accounts: Arc<Mutex<Vec<dev::APIAccount>>>,
    index: Arc<Mutex<dev::Index>>,

    ip_address: Arc<Mutex<String>>,
}

#[derive(Debug)]
pub enum APIError {
    /// API hasn't been initialized yet (logging in + making keys).
    ClientNotReady,
    /// This is useless?
    // BadRequest(String),
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

    pub async fn load(&mut self, credentials: Credentials) {
        self.credentials = credentials;
        self.ready = false;
        self.init().await;
        self.ready = true;
    }

    async fn get_ip() -> Result<String, reqwest::Error> {
        Ok(CLIENT.get(Self::IP_URL).send().await?.text().await?)
    }

    async fn init(&self) {
        let ip = Client::get_ip().await.unwrap();
        self.ip_address.lock().unwrap().push_str(&ip);

        for credential in self.credentials.0.iter() {
            let account = dev::APIAccount::login(credential, ip.clone()).await;
            self.accounts.lock().unwrap().push(account);
        }
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
    fn get(&self, url: String) -> Result<reqwest::RequestBuilder, APIError> {
        if !self.ready {
            return Err(APIError::ClientNotReady);
        }
        Ok(CLIENT.get(url).bearer_auth(&self.cycle()))
    }

    fn post(&self, url: String, body: String) -> Result<reqwest::RequestBuilder, APIError> {
        if !self.ready {
            return Err(APIError::ClientNotReady);
        }
        Ok(CLIENT.post(url).bearer_auth(&self.cycle()).body(body))
    }

    //                                                            //
    //                                                            //
    // --------------------------ENDPOINTS----------------------- //
    //                                                            //
    //                                                            //

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Clan Methods
    //_______________________________________________________________________
    pub async fn get_clan_warlog(
        &self,
        mut tag: String,
    ) -> Result<APIResponse<war_log::WarLog>, APIError> {
        tag = self.fix_tag(tag);
        let url = format!(
            "{}/clans/{}/warlog",
            Self::BASE_URL,
            urlencoding::encode(tag.as_str())
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_clans(
        &self,
        options: clan_search::ClanSearchOptions,
    ) -> Result<APIResponse<clan::Clan>, APIError> {
        let url =
            Url::parse_with_params(format!("{}/clans", Self::BASE_URL).as_str(), options.items)
                .unwrap();
        self.parse_json(self.get(url.to_string())).await
    }

    pub async fn get_current_war(&self, mut clan_tag: String) -> Result<war::War, APIError> {
        clan_tag = self.fix_tag(clan_tag);
        let url = format!(
            "{}/clans/{}/currentwar",
            Self::BASE_URL,
            urlencoding::encode(clan_tag.as_str())
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_clan(&self, mut clan_tag: String) -> Result<clan::Clan, APIError> {
        clan_tag = self.fix_tag(clan_tag);
        let url = format!(
            "{}/clans/{}",
            Self::BASE_URL,
            urlencoding::encode(clan_tag.as_str())
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
            urlencoding::encode(clan_tag.as_str())
        );
        self.parse_json(self.get(url)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Player Methods
    //_______________________________________________________________________
    pub async fn get_player(&self, mut tag: String) -> Result<player::Player, APIError> {
        tag = self.fix_tag(tag);
        let url = format!(
            "{}/players/{}",
            Self::BASE_URL,
            urlencoding::encode(tag.as_str())
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn verify_player_token(
        &self,
        mut tag: String,
        token: String,
    ) -> Result<player::PlayerToken, APIError> {
        tag = self.fix_tag(tag);
        let url = format!(
            "{}/players/{}/verifytoken",
            Self::BASE_URL,
            urlencoding::encode(tag.as_str())
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

    //                                                            //
    //                                                            //
    // --------------------------END POINTS-----------------------//
    //                                                            //
    //                                                            //

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

    /// Runs the future that implements `Send` and parses the reqwest response into a `APIResponse`.
    async fn parse_json<T: DeserializeOwned>(
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
                                Ok(serde_json::from_str(&text)
                            .expect(format!("Failure parsing json (please file a bug on the GitHub): {}", text).as_str()))
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
