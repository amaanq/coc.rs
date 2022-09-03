use chrono::TimeZone;
use serde::{Deserialize, Serialize};

use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarLog {
    pub result: Option<String>,
    end_time: String,
    pub team_size: i32,
    pub attacks_per_member: i8,
    pub clan: Clan,
    pub opponent: Opponent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub tag: String,
    pub name: String,
    pub badge_urls: BadgeUrls,
    pub clan_level: i8,
    pub attacks: i32,
    pub stars: i32,
    pub destruction_percentage: f32,
    pub exp_earned: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opponent {
    pub tag: Option<String>,
    pub name: Option<String>,
    pub badge_urls: BadgeUrls,
    pub clan_level: i32,
    pub stars: i32,
    pub destruction_percentage: f32,
}

impl WarLog {
    pub fn end_time(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_utc_datetime(
            &chrono::NaiveDateTime::parse_from_str(&self.end_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
        )
    }
}
