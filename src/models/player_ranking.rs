use serde::{Deserialize, Serialize};

use super::{badge_urls::BadgeUrls, clan::League};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRanking {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "expLevel")]
    exp_level: i32,

    #[serde(rename = "trophies")]
    trophies: i32,

    #[serde(rename = "attackWins")]
    attack_wins: i32,

    #[serde(rename = "defenseWins")]
    defense_wins: i32,

    #[serde(rename = "rank")]
    rank: i32,

    #[serde(rename = "previousRank")]
    previous_rank: Option<i32>,

    #[serde(rename = "versusTrophies")]
    versus_trophies: Option<i32>,

    #[serde(rename = "versusBattleWins")]
    versus_battle_wins: Option<i32>,

    #[serde(rename = "clan")]
    clan: Option<PlayerClan>,

    #[serde(rename = "league")]
    league: Option<League>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerClan {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "badgeUrls")]
    badge_urls: BadgeUrls,
}

impl PlayerRanking {
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn exp_level(&self) -> i32 {
        self.exp_level
    }
    pub fn trophies(&self) -> i32 {
        self.trophies
    }
    pub fn attack_wins(&self) -> i32 {
        self.attack_wins
    }
    pub fn defense_wins(&self) -> i32 {
        self.defense_wins
    }
    pub fn rank(&self) -> i32 {
        self.rank
    }
    pub fn clan(&self) -> Option<&PlayerClan> {
        self.clan.as_ref()
    }
}
