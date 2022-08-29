use regex::Regex;
use serde::{Deserialize, Serialize};

extern crate reqwest;

use crate::credentials::Credentials;
use crate::models::clan::{
    Clan, ClanMember, Label, League as ClanLeague, WarLeague as ClanWarLeague,
};
use crate::models::clan_ranking::ClanRanking;
use crate::models::clan_search::ClanSearchOptions;
use crate::models::current_war::War;
use crate::models::gold_pass::GoldPass;
use crate::models::leagues::{League, Season, WarLeague};
use crate::models::locations::{Local, Location};
use crate::models::paging::Paging;
use crate::models::player::{Player, PlayerToken};
use crate::models::player_ranking::{PlayerRanking, PlayerVersusRanking};
use crate::models::war_log::WarLog;

use reqwest::{RequestBuilder, Url};
use serde::de::DeserializeOwned;

use crate::dev::{self, APIAccount, CLIENT};
use std::error::Error;
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
    ClientNotReady,
    BadRequest, // this is useless?
    RequestFailed(reqwest::Error),
    BadResponse(String, reqwest::StatusCode),
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
            let account = APIAccount::login(credential, ip.clone()).await;
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
    pub async fn get_clan_warlog(&self, mut tag: String) -> Result<APIResponse<WarLog>, APIError> {
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
        options: ClanSearchOptions,
    ) -> Result<APIResponse<Clan>, APIError> {
        let url =
            Url::parse_with_params(format!("{}/clans", Self::BASE_URL).as_str(), options.items)
                .unwrap();
        self.parse_json(self.get(url.to_string())).await
    }

    pub async fn get_current_war(&self, mut clan_tag: String) -> Result<War, APIError> {
        clan_tag = self.fix_tag(clan_tag);
        let url = format!(
            "{}/clans/{}/currentwar",
            Self::BASE_URL,
            urlencoding::encode(clan_tag.as_str())
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_clan(&self, mut clan_tag: String) -> Result<Clan, APIError> {
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
    ) -> Result<APIResponse<ClanMember>, APIError> {
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
    pub async fn get_player(&self, mut tag: String) -> Result<Player, APIError> {
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
    ) -> Result<PlayerToken, APIError> {
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
    pub async fn get_leagues(&self) -> Result<APIResponse<ClanLeague>, APIError> {
        let url = format!("{}/leagues", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    // /leagues/{leagueId}/seasons/{seasonId}
    pub async fn get_league_season_rankings(
        &self,
        league_id: League,
        mut season_id: Season,
        paging: Paging,
    ) -> Result<APIResponse<PlayerRanking>, APIError> {
        if league_id != League::LegendLeague {
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

    pub async fn get_league(&self, league_id: League) -> Result<ClanLeague, APIError> {
        let url = format!("{}/leagues/{}", Self::BASE_URL, league_id as i32);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_league_seasons(
        &self,
        league_id: League,
    ) -> Result<APIResponse<Season>, APIError> {
        if league_id != League::LegendLeague {
            return Err(APIError::InvalidParameters(
                "This league does not have seasons, only League::LegendLeague has seasons"
                    .to_string(),
            ));
        }
        let url = format!("{}/leagues/{}/seasons", Self::BASE_URL, league_id as i32);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_war_league(&self, war_league: WarLeague) -> Result<ClanWarLeague, APIError> {
        let url = format!("{}/warleagues/{}", Self::BASE_URL, war_league as i32);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_war_leagues(&self) -> Result<APIResponse<ClanWarLeague>, APIError> {
        let url = format!("{}/warleagues", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Location Methods
    //_______________________________________________________________________

    pub async fn get_clan_rankings(
        &self,
        location: Local,
    ) -> Result<APIResponse<ClanRanking>, APIError> {
        let url = format!(
            "{}/locations/{}/rankings/clans",
            Self::BASE_URL,
            location as i32,
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_player_rankings(
        &self,
        location: Local,
    ) -> Result<APIResponse<PlayerRanking>, APIError> {
        let url = format!(
            "{}/locations/{}/rankings/players",
            Self::BASE_URL,
            location as i32,
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_versus_clan_rankings(
        &self,
        location: Local,
    ) -> Result<APIResponse<ClanRanking>, APIError> {
        let url = format!(
            "{}/locations/{}/rankings/clans-versus",
            Self::BASE_URL,
            location as i32,
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_versus_player_rankings(
        &self,
        location: Local,
    ) -> Result<APIResponse<PlayerVersusRanking>, APIError> {
        let url = format!(
            "{}/locations/{}/rankings/players-versus",
            Self::BASE_URL,
            location as i32,
        );
        self.parse_json(self.get(url)).await
    }

    pub async fn get_locations(&self) -> Result<APIResponse<Location>, APIError> {
        let url = format!("{}/locations", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_location(&self, location: Local) -> Result<Location, APIError> {
        let url = format!("{}/locations/{}", Self::BASE_URL, location as i32);
        self.parse_json(self.get(url)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Gold Pass Method
    //_______________________________________________________________________
    pub async fn get_goldpass(&self) -> Result<GoldPass, APIError> {
        let url = format!("{}{}", Self::BASE_URL, Self::GOLDPASS_ENDPOINT);
        self.parse_json(self.get(url)).await
    }

    //‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    // Label Methods
    //_______________________________________________________________________
    pub async fn get_player_labels(&self) -> Result<APIResponse<Label>, APIError> {
        let url = format!("{}/labels/players", Self::BASE_URL);
        self.parse_json(self.get(url)).await
    }

    pub async fn get_clan_labels(&self) -> Result<APIResponse<Label>, APIError> {
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

    async fn parse_json<T: DeserializeOwned>(
        &self,
        rb: Result<RequestBuilder, APIError>,
    ) -> Result<T, APIError> {
        match rb {
            Ok(rb) => match rb.send().await {
                Ok(resp) => match resp.status() {
                    reqwest::StatusCode::OK => {
                        let text = resp.text().await.unwrap();
                        Ok(serde_json::from_str(&text)
                            .expect(format!("Could not parse json: {}", text).as_str()))
                    }
                    _ => {
                        let status = resp.status();
                        Err(APIError::BadResponse(resp.text().await.unwrap(), status))
                    }
                },
                Err(e) => Err(APIError::RequestFailed(e)),
            },
            Err(_) => return Err(APIError::BadRequest),
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
    pub paging: Paging,
}
