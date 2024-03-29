use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use async_recursion::async_recursion;
use dashmap::DashMap;
use parking_lot::Mutex;
use reqwest::{RequestBuilder, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[cfg(feature = "cos")]
use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    credentials::{Credential, Credentials},
    dev::{self, CLIENT},
    error::APIError,
    models::{
        clan, clan_capital, clan_search, gold_pass, labels, leagues, location, paging, player,
        rankings, season, war, war_log,
    },
    util::LogicLong,
};

#[derive(Clone, Debug, Default)]
pub struct Client {
    ready: Arc<AtomicBool>,
    pub(crate) accounts: Arc<DashMap<Credential, dev::APIAccount>>,

    account_index: Arc<AtomicUsize>,
    key_index: Arc<AtomicUsize>,

    ip_address: Arc<Mutex<String>>,

    #[cfg(feature = "cos")]
    pub(crate) is_cos_logged_in: Arc<AtomicBool>,
}

impl Client {
    const BASE_URL: &'static str = "https://api.clashofclans.com/v1";

    /// Returns a [`Client`]
    ///
    /// # Errors
    ///
    /// This function will return an error if the credentials are invalid
    pub async fn new(credentials: Credentials) -> anyhow::Result<Self> {
        let client = Self {
            ready: Arc::new(AtomicBool::new(false)),

            accounts: Arc::new(DashMap::new()),

            account_index: Arc::new(AtomicUsize::new(0)),
            key_index: Arc::new(AtomicUsize::new(0)),

            ip_address: Arc::new(Mutex::new(String::new())),

            #[cfg(feature = "cos")]
            is_cos_logged_in: Arc::new(AtomicBool::new(false)),
        };

        client.init(credentials).await?;
        client.ready.store(true, Ordering::Relaxed);
        Ok(client)
    }

    /// Called when the client is created to initialize every credential.
    async fn init(&self, credentials: Credentials) -> anyhow::Result<()> {
        let tasks = credentials.0.into_iter().map(dev::APIAccount::login);

        let accounts =
            futures::future::join_all(tasks).await.into_iter().collect::<Result<Vec<_>, _>>()?;

        *self.ip_address.lock() = accounts[0].1.clone();

        for (account, _) in accounts {
            self.accounts.insert(account.credential.clone(), account);
        }

        Ok(())
    }

    /// Called when an IP address change is detected
    pub(crate) async fn reinit(&self) -> anyhow::Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!("reinitializing client");

        self.ready.store(false, Ordering::Relaxed);

        let accounts = self.accounts.iter().map(|account| account.clone()).collect::<Vec<_>>();

        for mut account in accounts {
            account.re_login().await?;

            // update the account in the DashMap
            self.accounts.insert(account.credential.clone(), account);
        }

        self.ready.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Here you can create a client yourself and load them here later (for example .env parsing)
    ///
    /// # Errors
    ///
    /// This function will return an error if the credentials are invalid
    ///
    /// # Example
    /// ```no_run
    /// use coc_rs::{api::Client, credentials::Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = Client::new(None);
    ///     let credentials = Credentials::builder()
    ///         .add_credential("email", "password")
    ///         .add_credential("email2", "password2")
    ///         .build();
    ///     client.load(credentials).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn load(&self, credentials: Credentials) -> anyhow::Result<()> {
        #[cfg(feature = "tracing")]
        tracing::trace!(credentials = ?credentials, "Loading credentials");

        self.ready.store(false, Ordering::Relaxed);
        self.init(credentials).await?;
        self.ready.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// This is purely for diagnostics, it's not used anywhere else.
    ///
    /// # Example
    /// ```no_run
    /// use coc_rs::Client;
    ///
    /// let credentials = Credentials::builder()
    ///     .add_credential("email", "password")
    ///     .add_credential("email2", "password2")
    ///     .build();
    /// let client = Client::new(credentials);
    /// client.debug_keys().await;
    /// ```
    #[cfg(feature = "tracing")]
    pub fn debug_keys(&self) {
        self.accounts.iter().for_each(|account| {
            account.keys.keys.iter().for_each(|key| {
                tracing::debug!(key = %key.key, key.id=%key.id, key.name=%key.name);
            });
        });
    }

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                       HTTP Methods                       │
    //         ╰──────────────────────────────────────────────────────────╯

    pub(crate) fn get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::RequestBuilder, APIError> {
        if !self.ready.load(Ordering::Relaxed) {
            return Err(APIError::ClientNotReady);
        }
        Ok(CLIENT.get(url).bearer_auth(self.get_next_key()))
    }

    pub(crate) fn post<U: reqwest::IntoUrl, T: Into<reqwest::Body>>(
        &self,
        url: U,
        body: T,
    ) -> Result<reqwest::RequestBuilder, APIError> {
        if !self.ready.load(Ordering::Relaxed) {
            return Err(APIError::ClientNotReady);
        }
        Ok(CLIENT.post(url).bearer_auth(self.get_next_key()).body(body))
    }

    /// To allow usage without a client being ready
    #[cfg(feature = "cos")]
    pub(crate) fn cos_get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::RequestBuilder, APIError> {
        let mut headers = HeaderMap::new();
        headers.insert("authority", HeaderValue::from_str("api.clashofstats.com")?);
        headers.insert("method", HeaderValue::from_str("GET")?);
        headers.insert("scheme", HeaderValue::from_str("https")?);
        headers.insert(
            "accept",
            HeaderValue::from_str("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")?,
        );
        headers.insert(
            "accept-language",
            HeaderValue::from_str("en-US,en;q=0.9,zh-CN;q=0.8,z;q=0.7")?,
        );
        headers.insert(
            "sec-ch-ua",
            HeaderValue::from_str(
                "\"Not/A)Brand\";v=\"99\", \"Google Chrome\";v=\"103\", \"Chromium\";v=\"103\"",
            )?,
        );
        headers.insert("sec-ch-ua-platform", HeaderValue::from_str("\"Windows\"")?);
        headers.insert("upgrade-insecure-requests", HeaderValue::from_str("1")?);
        headers.insert(
            "user-agent",
            HeaderValue::from_str("Mozilla/5.0 (X11; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")?,
        );

        Ok(CLIENT.get(url).headers(headers))
    }

    /// To allow usage without a client being ready
    #[cfg(feature = "cos")]
    pub(crate) fn cos_post<U: reqwest::IntoUrl, T: Into<reqwest::Body>>(
        &self,
        url: U,
        body: T,
    ) -> Result<reqwest::RequestBuilder, APIError> {
        let mut headers = HeaderMap::new();
        headers.insert("authority", HeaderValue::from_str("api.clashofstats.com")?);
        headers.insert("method", HeaderValue::from_str("POST")?);
        headers.insert("scheme", HeaderValue::from_str("https")?);
        headers.insert("accept", HeaderValue::from_str("application/json, text/plain, */*")?);
        headers.insert("accept-encoding", HeaderValue::from_str("gzip, deflate, br")?);
        headers.insert(
            "accept-language",
            HeaderValue::from_str("en-US,en;q=0.9,zh-CN;q=0.8,z;q=0.7")?,
        );
        headers.insert("content-type", HeaderValue::from_str("application/json;charset=UTF-8")?);
        headers.insert(
            "sec-ch-ua",
            HeaderValue::from_str(
                "\"Not/A)Brand\";v=\"99\", \"Google Chrome\";v=\"103\", \"Chromium\";v=\"103\"",
            )?,
        );
        headers.insert("sec-ch-ua-platform", HeaderValue::from_str("\"Windows\"")?);
        headers.insert("upgrade-insecure-requests", HeaderValue::from_str("1")?);
        headers.insert(
            "user-agent",
            HeaderValue::from_str("Mozilla/5.0 (X11; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")?,
        );

        Ok(CLIENT.post(url).body(body).headers(headers))
    }

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                       Clan Methods                       │
    //         ╰──────────────────────────────────────────────────────────╯

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_clan_warlog(
        &self,
        clan_tag: &str,
    ) -> Result<APIResponse<war_log::WarLog>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_clan_warlog({})", clan_tag);
        let clan_tag = clan_tag.parse::<LogicLong>()?.to_string();
        let url = format!("{}/clans/{}/warlog", Self::BASE_URL, urlencoding::encode(&clan_tag));
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_clans(
        &self,
        options: clan_search::ClanSearchOptions,
    ) -> Result<APIResponse<clan::Clan>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_clans({})", options);
        let url = Url::parse_with_params(&format!("{}/clans", Self::BASE_URL), options.items)?;
        self.parse_json(self.get(url.to_string()), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_current_war(&self, clan_tag: &str) -> Result<war::War, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_current_war({})", clan_tag);
        let clan_tag = clan_tag.parse::<LogicLong>()?.to_string();
        let url = format!("{}/clans/{}/currentwar", Self::BASE_URL, urlencoding::encode(&clan_tag));
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_clan(&self, clan_tag: &str) -> Result<clan::Clan, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_clan({})", clan_tag);
        let clan_tag = clan_tag.parse::<LogicLong>()?.to_string();
        let url = format!("{}/clans/{}", Self::BASE_URL, urlencoding::encode(&clan_tag));
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_clan_members(
        &self,
        clan_tag: &str,
    ) -> Result<APIResponse<clan::ClanMember>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_clan_members({})", clan_tag);
        let clan_tag = clan_tag.parse::<LogicLong>()?.to_string();
        let url = format!("{}/clans/{}/members", Self::BASE_URL, urlencoding::encode(&clan_tag));
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_clan_capital_raid_seasons(
        &self,
        clan_tag: &str,
    ) -> Result<APIResponse<clan_capital::ClanCapitalRaidSeason>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_clan_capital_raid_seasons({})", clan_tag);
        let clan_tag = clan_tag.parse::<LogicLong>()?.to_string();
        let url = format!(
            "{}/clans/{}/capitalraidseasons",
            Self::BASE_URL,
            urlencoding::encode(&clan_tag)
        );
        self.parse_json(self.get(url), false).await
    }

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                      Player Methods                      │
    //         ╰──────────────────────────────────────────────────────────╯

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_player(&self, player_tag: &str) -> Result<player::Player, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_player({})", player_tag);
        let player_tag = player_tag.parse::<LogicLong>()?.to_string();
        let url = format!("{}/players/{}", Self::BASE_URL, urlencoding::encode(&player_tag));
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn verify_player_token(
        &self,
        player_tag: &str,
        token: &str,
    ) -> Result<player::PlayerToken, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("verify_player_token({}, {})", player_tag, token);
        let player_tag = player_tag.parse::<LogicLong>()?.to_string();
        let url =
            format!("{}/players/{}/verifytoken", Self::BASE_URL, urlencoding::encode(&player_tag));
        let token = format!("{{\"token\":\"{token}\"}}");
        self.parse_json(self.post(url, token), false).await
    }

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                      League Methods                      │
    //         ╰──────────────────────────────────────────────────────────╯

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_leagues(&self) -> Result<APIResponse<leagues::League>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_leagues()");
        let url = format!("{}/leagues", Self::BASE_URL);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_league_season_rankings(
        &self,
        league_id: leagues::LeagueKind,
        season_id: season::Season,
        paging: paging::Paging,
    ) -> Result<APIResponse<rankings::PlayerRanking>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_league_season_rankings({}, {}, {})", league_id, season_id, paging);
        if league_id != leagues::LeagueKind::LegendLeague {
            return Err(APIError::InvalidParameters(
                "This league does not have seasons, only League::LegendLeague has seasons"
                    .to_string(),
            ));
        }
        let mut url =
            format!("{}/leagues/{}/seasons/{season_id}", Self::BASE_URL, league_id as i32);
        if paging.is_some() {
            url = Url::parse_with_params(&url, paging.to_vec())?.to_string();
        }
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_league(
        &self,
        league_id: leagues::LeagueKind,
    ) -> Result<leagues::League, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_league({})", league_id);
        let url = format!("{}/leagues/{}", Self::BASE_URL, league_id as i32);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_league_seasons(
        &self,
        league_id: leagues::LeagueKind,
    ) -> Result<APIResponse<season::Season>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_league_seasons({})", league_id);
        if league_id != leagues::LeagueKind::LegendLeague {
            return Err(APIError::InvalidParameters(
                "This league does not have seasons, only League::LegendLeague has seasons"
                    .to_string(),
            ));
        }
        let url = format!("{}/leagues/{}/seasons", Self::BASE_URL, league_id as i32);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_war_league(
        &self,
        war_league: leagues::WarLeagueKind,
    ) -> Result<leagues::WarLeague, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_war_league({})", war_league);
        let url = format!("{}/warleagues/{}", Self::BASE_URL, war_league as i32);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_war_leagues(&self) -> Result<APIResponse<leagues::WarLeague>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_war_leagues()");
        let url = format!("{}/warleagues", Self::BASE_URL);
        self.parse_json(self.get(url), false).await
    }

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                     Location Methods                     │
    //         ╰──────────────────────────────────────────────────────────╯

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_clan_rankings(
        &self,
        location: location::Local,
    ) -> Result<APIResponse<rankings::ClanRanking>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_clan_rankings({})", location);
        let url = format!("{}/locations/{}/rankings/clans", Self::BASE_URL, location as i32);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_player_rankings(
        &self,
        location: location::Local,
    ) -> Result<APIResponse<rankings::PlayerRanking>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_player_rankings({})", location);
        let url = format!("{}/locations/{}/rankings/players", Self::BASE_URL, location as i32);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_versus_clan_rankings(
        &self,
        location: location::Local,
    ) -> Result<APIResponse<rankings::ClanRanking>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_versus_clan_rankings({})", location);
        let url = format!("{}/locations/{}/rankings/clans-versus", Self::BASE_URL, location as i32);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_versus_player_rankings(
        &self,
        location: location::Local,
    ) -> Result<APIResponse<rankings::PlayerVersusRanking>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_versus_player_rankings({})", location);
        let url =
            format!("{}/locations/{}/rankings/players-versus", Self::BASE_URL, location as i32);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_locations(&self) -> Result<APIResponse<location::Location>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_locations()");
        let url = format!("{}/locations", Self::BASE_URL);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_location(
        &self,
        location: location::Local,
    ) -> Result<location::Location, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_location({})", location);
        let url = format!("{}/locations/{}", Self::BASE_URL, location as i32);
        self.parse_json(self.get(url), false).await
    }

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                     Gold Pass Method                     │
    //         ╰──────────────────────────────────────────────────────────╯

    /// # Errors
    ///
    /// This function will return an error if the request fails
    pub async fn get_goldpass(&self) -> Result<gold_pass::GoldPass, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_goldpass()");
        let url = format!("{}/goldpass/seasons/current", Self::BASE_URL);
        self.parse_json(self.get(url), false).await
    }

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                      Label Methods                       │
    //         ╰──────────────────────────────────────────────────────────╯

    /// # Errors
    ///
    /// This function will return an error if the request fails.
    pub async fn get_player_labels(&self) -> Result<APIResponse<labels::PlayerLabel>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_player_labels()");
        let url = format!("{}/labels/players", Self::BASE_URL);
        self.parse_json(self.get(url), false).await
    }

    /// # Errors
    ///
    /// This function will return an error if the request fails.
    pub async fn get_clan_labels(&self) -> Result<APIResponse<labels::ClanLabel>, APIError> {
        #[cfg(feature = "tracing")]
        tracing::trace!("get_clan_labels()");
        let url = format!("{}/labels/clans", Self::BASE_URL);
        self.parse_json(self.get(url), false).await
    }

    /// Runs the future that implements `Send` and parses the reqwest response into an
    /// `APIResponse`.
    ///
    /// # Panics
    ///
    /// Panics if the JSON parsing fails for some odd reason. This is a bug and should be reported.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails.
    #[async_recursion]
    pub(crate) async fn parse_json<T: DeserializeOwned>(
        &self,
        rb: Result<RequestBuilder, APIError>,
        is_retry_and_not_cos: bool,
    ) -> Result<T, APIError> {
        match rb {
            Ok(rb) => {
                let cloned_rb = rb.try_clone();
                match rb.send().await {
                    Ok(resp) => {
                        match resp.status() {
                            reqwest::StatusCode::OK => {
                                let text = resp.text().await?;
                                Ok(serde_json::from_str(&text).unwrap_or_else(|e| panic!("Failure parsing json (please file a bug on the GitHub): {text}\nError: {e}")))
                            }
                            // 400
                            reqwest::StatusCode::BAD_REQUEST => Err(APIError::BadParameters),
                            // 403 - likely means the IP address has changed, let's reinit the
                            // client then and try this again
                            reqwest::StatusCode::FORBIDDEN => {
                                if is_retry_and_not_cos {
                                    #[cfg(feature = "tracing")]
                                    tracing::debug!("403 Forbidden, but already retried, try checking your credentials?");
                                    Err(APIError::AccessDenied)
                                } else {
                                    if let Err(e) = self.reinit().await {
                                        return Err(APIError::LoginFailed(e.to_string()));
                                    }
                                    if let Some(rb) = cloned_rb {
                                        self.parse_json(Ok(rb), true).await
                                    } else {
                                        Err(APIError::AccessDenied)
                                    }
                                }
                            }
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
                                #[cfg(feature = "tracing")]
                                tracing::debug!("Unknown status code: {}", status);
                                Err(APIError::BadResponse(resp.text().await?, status))
                            }
                        }
                    }
                    Err(e) => Err(APIError::RequestFailed(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn get_next_key(&self) -> String {
        // increment key_token_index, unless it would be larger than the account's token size (10),
        // then reset to 0 and increment key_account_index

        let mut account_index = self.account_index.load(Ordering::Relaxed);
        let mut key_index = self.key_index.load(Ordering::Relaxed);

        let accounts = self.accounts.iter().collect::<Vec<_>>();
        let size_of_keys = accounts[account_index].keys.len().min(10);

        // if we're at the end of this account's keys..
        if key_index == size_of_keys - 1 {
            // reset token index anyways
            key_index = 0;
            // ..and at the end of the accounts
            if account_index == (accounts.len() - 1) {
                // then we've reached end of accounts, go back to first account
                account_index = 0;
            } else {
                // otherwise, just increment account index
                account_index += 1;
            }
        } else {
            // otherwise, just increment token index
            key_index += 1;
        }

        let token = accounts
            .get(account_index)
            .unwrap_or_else(|| {
                #[cfg(feature = "tracing")]
                tracing::warn!("No account found at index {account_index}");
                panic!("No account found at index {account_index}")
            })
            .keys
            .keys
            .get(key_index)
            .unwrap_or_else(|| {
                #[cfg(feature = "tracing")]
                tracing::warn!("No key found at index {key_index}");
                panic!("No key found at index {key_index}");
            })
            .clone();

        self.account_index.store(account_index, Ordering::Relaxed);
        self.key_index.store(key_index, Ordering::Relaxed);

        token.key
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse<T> {
    pub items: Vec<T>,
    pub paging: paging::Paging,
}
