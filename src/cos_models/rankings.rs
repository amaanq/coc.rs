#[cfg(feature = "cos")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "cos")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanRanking {
    pub tag: String,
    pub name: String,
    pub value: i32,
    pub rank: i32,
    pub badge: String,
    pub members: i32,
    pub level: i32,
    pub location_id: Option<String>,
    pub other_ranking: Option<OtherRanking>,
}

#[cfg(feature = "cos")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherRanking {
    pub location_id: String,
    pub rank: i32,
}

#[cfg(feature = "cos")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRanking {
    pub tag: String,
    pub name: String,
    pub value: i32,
    pub rank: i32,
    pub town_hall_level: i8,
    pub town_hall_weapon_level: Option<i8>,
    pub builder_hall_level: Option<i8>,
    pub clan: Option<Clan>,
    pub clan_tag: Option<String>,
    pub character_id: String,
}

#[cfg(feature = "cos")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub name: String,
    pub tag: String,
    pub badge: String,
}

#[cfg(feature = "cos")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegendsRanking {
    pub tag: String,
    pub name: String,
    pub trophies: i32,
    pub value: i32,
    pub rank: i32,
    pub previous_rank: Option<i32>,
    pub clan: Option<Clan>,
    pub clan_tag: Option<String>,
    pub character_id: String,
}
