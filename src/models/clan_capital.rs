use chrono::TimeZone;
use serde::{Deserialize, Serialize};

use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeason {
    pub state: State,
    start_time: String,
    end_time: String,
    pub capital_total_loot: i32,
    pub raids_completed: i32,
    pub total_attacks: i32,
    pub enemy_districts_destroyed: i32,
    pub offensive_reward: i32,
    pub defensive_reward: i32,
    pub attack_log: Vec<AttackLog>,
    pub defense_log: Vec<DefenseLog>,
    pub members: Option<Vec<Member>>,
}

impl ClanCapitalRaidSeason {
    #[must_use]
    pub fn start_time(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_utc_datetime(
            &chrono::NaiveDateTime::parse_from_str(&self.start_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
        )
    }

    #[must_use]
    pub fn end_time(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_utc_datetime(
            &chrono::NaiveDateTime::parse_from_str(&self.end_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct District {
    pub id: i32,
    pub name: DistrictName,
    pub destruction_percent: i32,
    pub attack_count: i32,
    pub total_looted: i32,
    pub attacks: Option<Vec<Attack>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackLog {
    pub defender: Clan,
    pub attack_count: i32,
    pub district_count: i32,
    pub districts_destroyed: i32,
    pub districts: Vec<District>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseLog {
    pub attacker: Clan,
    pub attack_count: i32,
    pub district_count: i32,
    pub districts_destroyed: i32,
    pub districts: Vec<District>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub attacker: Attacker,
    pub destruction_percent: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attacker {
    pub tag: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub tag: String,
    pub name: String,
    pub level: i32,
    pub badge_urls: BadgeUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub tag: String,
    pub name: String,
    pub attacks: i32,
    pub attack_limit: i32,
    pub bonus_attack_limit: i32,
    pub capital_resources_looted: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistrictName {
    #[serde(rename = "Balloon Lagoon")]
    BalloonLagoon,
    #[serde(rename = "Barbarian Camp")]
    BarbarianCamp,
    #[serde(rename = "Builder's Workshop")]
    BuilderSWorkshop,
    #[serde(rename = "Capital Peak")]
    CapitalPeak,
    #[serde(rename = "Dragon Cliffs")]
    DragonCliffs,
    #[serde(rename = "Golem Quarry")]
    GolemQuarry,
    #[serde(rename = "Wizard Valley")]
    WizardValley,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum State {
    Ended,
    Ongoing,
}
