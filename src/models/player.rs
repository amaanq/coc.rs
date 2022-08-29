use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub tag: String,
    pub name: String,
    pub town_hall_level: i8,
    pub town_hall_weapon_level: Option<i8>,
    pub exp_level: i32,
    pub trophies: i32,
    pub best_trophies: i32,
    pub war_stars: i32,
    pub attack_wins: i32,
    pub defense_wins: i32,
    pub builder_hall_level: Option<i8>,
    pub versus_trophies: i32,
    pub best_versus_trophies: i32,
    pub versus_battle_wins: i32,
    pub role: Option<clan::Role>,
    pub war_preference: Option<WarPreference>,
    pub donations: i32,
    pub donations_received: i32,
    pub clan_capital_contributions: i32,
    pub clan: Option<PlayerClan>,
    pub league: Option<leagues::League>,
    pub legend_statistics: Option<LegendStatistics>,
    pub achievements: Vec<Achievement>,
    pub versus_battle_win_count: i32,
    pub labels: Vec<labels::PlayerLabel>,
    pub troops: Vec<Troop>,
    pub heroes: Vec<Hero>,
    pub spells: Vec<Spell>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WarPreference {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hero {
    pub name: String,
    pub level: i32,
    pub max_level: i32,
    pub village: Village,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub name: String,
    pub stars: i32,
    pub value: i32,
    pub target: i32,
    pub info: String,
    pub completion_info: Option<String>,
    pub village: Village,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClan {
    pub tag: String,
    pub name: String,
    pub clan_level: i8,
    pub badge_urls: badge_urls::BadgeUrls,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    pub name: String,
    pub level: i32,
    pub max_level: i32,
    pub village: Village,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Troop {
    pub name: String,
    pub level: i32,
    pub max_level: i32,
    pub village: Village,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Village {
    #[serde(rename = "builderBase")]
    BuilderBase,

    #[serde(rename = "home")]
    HomeVillage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerToken {
    pub tag: String,
    pub token: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegendStatistics {
    pub legend_trophies: i32,
    pub previous_season: season::SeasonData,
    pub best_season: season::SeasonData,
    pub current_season: season::SeasonData,
}

impl Player {
    pub fn game_link(&self) -> String {
        format!(
            "https://link.clashofclans.com/en?action=OpenPlayerProfile&tag={}",
            self.tag.replace("#", "")
        )
    }
    pub fn clash_of_stats_link(&self) -> String {
        format!(
            "https://www.clashofstats.com/players/{}/summary",
            self.tag.replace("#", "")
        )
    }

    pub fn chocolate_clash_link(&self) -> String {
        format!(
            "https://cc.chocolateclash.com/cc_n/member.php?tag={}",
            self.tag.replace("#", "")
        )
    }
}
