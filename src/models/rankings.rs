use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRanking {
    pub tag: String,
    pub name: String,
    pub exp_level: i32,
    pub trophies: i32,
    pub attack_wins: i32,
    pub defense_wins: i32,
    pub rank: i32,
    pub previous_rank: Option<i32>,
    pub clan: Option<PlayerRankingClan>,
    pub league: Option<leagues::League>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerVersusRanking {
    pub tag: String,
    pub name: String,
    pub exp_level: i32,
    pub rank: i32,
    pub previous_rank: Option<i32>,
    pub versus_trophies: Option<i32>,
    pub versus_battle_wins: Option<i32>,
    pub clan: Option<PlayerRankingClan>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRankingClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: badge_urls::BadgeUrls,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanRanking {
    pub tag: String,
    pub name: String,
    pub location: location::Location,
    pub badge_urls: badge_urls::BadgeUrls,
    pub clan_level: i8,
    pub members: i32,
    pub clan_points: Option<i32>,
    pub clan_versus_points: Option<i32>,
    pub rank: i32,
    pub previous_rank: i32,
}
