use serde::Deserialize;
use serde::Serialize;

use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarLog {
    result: Option<String>,
    end_time: String,
    team_size: i32,
    attacks_per_member: i8,
    clan: Clan,
    opponent: Opponent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    tag: String,
    name: String,
    badge_urls: BadgeUrls,
    clan_level: i8,
    attacks: i32,
    stars: i32,
    destruction_percentage: f32,
    exp_earned: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opponent {
    pub tag: Option<String>,
    pub name: Option<String>,
    pub badge_urls: BadgeUrls,
    pub clan_level: i64,
    pub stars: i64,
    pub destruction_percentage: f32,
}
