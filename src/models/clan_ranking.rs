use serde::{Deserialize, Serialize};

use super::{badge_urls::BadgeUrls, locations::Location};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClanRanking {
    #[serde(rename = "tag")]
    tag: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "location")]
    location: Location,
    #[serde(rename = "badgeUrls")]
    badge_urls: BadgeUrls,
    #[serde(rename = "clanLevel")]
    clan_level: i8,
    #[serde(rename = "members")]
    members: i32,
    #[serde(rename = "clanPoints")]
    clan_points: Option<i32>,
    #[serde(rename = "clanVersusPoints")]
    clan_versus_points: Option<i32>,
    #[serde(rename = "rank")]
    rank: i32,
    #[serde(rename = "previousRank")]
    previous_rank: i32,
}

impl ClanRanking {
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn location(&self) -> &Location {
        &self.location
    }
    pub fn badge_urls(&self) -> &BadgeUrls {
        &self.badge_urls
    }
    pub fn clan_level(&self) -> i8 {
        self.clan_level
    }
    pub fn members(&self) -> i32 {
        self.members
    }
    pub fn clan_points(&self) -> i32 {
        self.clan_points.unwrap_or_default()
    }
    pub fn rank(&self) -> i32 {
        self.rank
    }
    pub fn previous_rank(&self) -> i32 {
        self.previous_rank
    }
}
