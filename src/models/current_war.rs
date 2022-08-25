use crate::models::badge_urls::BadgeUrls;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct War {
    pub state: String,
    pub team_size: Option<i32>,
    pub attacks_per_member: Option<i8>,
    pub preparation_start_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub clan: Option<WarClan>,
    pub opponent: Option<WarClan>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarClan {
    pub tag: Option<String>,
    pub name: Option<String>,
    pub badge_urls: BadgeUrls,
    pub clan_level: Option<i8>,
    pub attacks: Option<i32>,
    pub stars: Option<i32>,
    pub destruction_percentage: Option<f64>,
    pub members: Option<Vec<Member>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub tag: String,
    pub name: String,
    pub townhall_level: i8,
    pub map_position: i32,
    pub attacks: Option<Vec<Attack>>,
    pub opponent_attacks: i32,
    pub best_opponent_attack: Option<Attack>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub attacker_tag: String,
    pub defender_tag: String,
    pub stars: i32,
    pub destruction_percentage: f32,
    pub order: i32,
    pub duration: i32,
}
