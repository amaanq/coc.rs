use chrono::TimeZone;
use serde::{Deserialize, Serialize};

use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct War {
    pub state: String,
    pub team_size: Option<i32>,
    pub attacks_per_member: Option<i8>,
    preparation_start_time: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    pub clan: Option<WarClan>,
    pub opponent: Option<WarClan>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub attacker_tag: String,
    pub defender_tag: String,
    pub stars: i32,
    pub destruction_percentage: f32,
    pub order: i32,
    pub duration: i32,
}

impl War {
    #[must_use]
    pub fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.start_time.as_ref().map(|start_time| {
            chrono::Utc.from_utc_datetime(
                &chrono::NaiveDateTime::parse_from_str(start_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
            )
        })
    }

    #[must_use]
    pub fn end_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.end_time.as_ref().map(|end_time| {
            chrono::Utc.from_utc_datetime(
                &chrono::NaiveDateTime::parse_from_str(end_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
            )
        })
    }

    #[must_use]
    pub fn preparation_start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.preparation_start_time.as_ref().map(|preparation_start_time| {
            chrono::Utc.from_utc_datetime(
                &chrono::NaiveDateTime::parse_from_str(preparation_start_time, "%Y%m%dT%H%M%S.%fZ")
                    .unwrap(),
            )
        })
    }
}
