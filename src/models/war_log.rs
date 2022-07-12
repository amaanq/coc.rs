use serde::Deserialize;
use serde::Serialize;

use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Serialize, Deserialize)]
pub struct WarLog {
    #[serde(rename = "result" )]
    result: String,
    #[serde(rename = "endTime" )]
    end_time: String,
    #[serde(rename = "teamSize" )]
    team_size: i32,
    #[serde(rename = "attacksPerMember" )]
    attacks_per_member: i8,
    #[serde(rename = "clan")]
    clan: Clan,
    #[serde(rename = "opponent")]
    opponent: Opponent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clan {
    #[serde(rename = "tag")]
    tag: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "badgeUrls")]
    badge_urls: BadgeUrls,
    #[serde(rename = "clanLevel")]
    clan_level: i8,
    #[serde(rename = "attacks")]
    attacks: i32,
    #[serde(rename = "stars")]
    stars: i32,
    #[serde(rename = "destructionPercentage")]
    destruction_percentage: f32,
    #[serde(rename = "expEarned")]
    exp_earned: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opponent {
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "badgeUrls")]
    pub badge_urls: BadgeUrls,
    #[serde(rename = "clanLevel")]
    pub clan_level: i64,
    #[serde(rename = "stars")]
    pub stars: i64,
    #[serde(rename = "destructionPercentage")]
    pub destruction_percentage: f32,
}
