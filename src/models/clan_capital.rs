use chrono::TimeZone;
use serde::{Deserialize, Serialize};

use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeason {
    pub state: State,
    start_time: String,
    end_time: String,
    pub capital_total_loot: i64,
    pub raids_completed: i64,
    pub total_attacks: i64,
    pub enemy_districts_destroyed: i64,
    pub offensive_reward: i64,
    pub defensive_reward: i64,
    pub attack_log: Vec<AttackLog>,
    pub defense_log: Vec<DefenseLog>,
    pub members: Option<Vec<Member>>,
}

impl ClanCapitalRaidSeason {
    pub fn start_time(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_utc_datetime(
            &chrono::NaiveDateTime::parse_from_str(&self.start_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
        )
    }

    pub fn end_time(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_utc_datetime(
            &chrono::NaiveDateTime::parse_from_str(&self.end_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct District {
    pub id: i64,
    pub name: DistrictName,
    pub destruction_percent: i64,
    pub attack_count: i64,
    pub total_looted: i64,
    pub attacks: Option<Vec<Attack>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackLog {
    pub defender: Defender,
    pub attack_count: i64,
    pub district_count: i64,
    pub districts_destroyed: i64,
    pub districts: Vec<District>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseLog {
    pub attacker: Defender,
    pub attack_count: i64,
    pub district_count: i64,
    pub districts_destroyed: i64,
    pub districts: Vec<District>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub attacker: Attacker,
    pub destruction_percent: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attacker {
    pub tag: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defender {
    pub tag: String,
    pub name: String,
    pub level: i64,
    pub badge_urls: BadgeUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub tag: String,
    pub name: String,
    pub attacks: i64,
    pub attack_limit: i64,
    pub bonus_attack_limit: i64,
    pub capital_resources_looted: i64,
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
}
