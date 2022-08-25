use serde::{Deserialize, Serialize};

use super::{badge_urls::BadgeUrls, locations::Location};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanRanking {
    pub tag: String,
    pub name: String,
    pub location: Location,
    pub badge_urls: BadgeUrls,
    pub clan_level: i8,
    pub members: i32,
    pub clan_points: Option<i32>,
    pub clan_versus_points: Option<i32>,
    pub rank: i32,
    pub previous_rank: i32,
}
