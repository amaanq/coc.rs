use serde::{Deserialize, Serialize};

use super::{badge_urls::BadgeUrls, clan::League};

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
    pub clan: Option<PlayerClan>,
    pub league: Option<League>,
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
    pub clan: Option<PlayerClan>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: BadgeUrls,
}
