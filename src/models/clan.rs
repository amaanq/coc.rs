use crate::models::badge_urls::BadgeUrls;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub tag: String,
    pub name: String,
    #[serde(rename = "type")]
    pub privacy: Privacy,
    pub description: Option<String>,
    pub location: Option<location::Location>,
    pub badge_urls: BadgeUrls,
    pub clan_level: i8,
    pub clan_points: i32,
    pub clan_versus_points: i32,
    pub required_trophies: i32,
    pub war_frequency: WarFrequency,
    pub war_win_streak: i32,
    pub war_wins: i32,
    pub war_ties: Option<i32>,
    pub war_losses: Option<i32>,
    pub is_war_log_public: bool,
    pub war_league: leagues::WarLeague,
    pub members: i32,
    pub member_list: Option<Vec<ClanMember>>,
    pub labels: Vec<labels::ClanLabel>,
    pub required_versus_trophies: i32,
    pub required_townhall_level: i8,
    pub clan_capital: Option<ClanCapital>,
    pub chat_language: Option<ChatLanguage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Privacy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "inviteOnly")]
    InviteOnly,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WarFrequency {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "moreThanOncePerWeek")]
    MoreThanOncePerWeek,
    #[serde(rename = "oncePerWeek")]
    OncePerWeek,
    #[serde(rename = "lessThanOncePerWeek")]
    LessThanOncePerWeek,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "any")]
    Any,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatLanguage {
    pub id: i32,
    pub name: String,
    pub language_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanMember {
    pub tag: String,
    pub name: String,
    pub role: Role,
    pub exp_level: i32,
    pub league: leagues::League,
    pub trophies: i32,
    pub versus_trophies: i32,
    pub clan_rank: i32,
    pub previous_clan_rank: i32,
    pub donations: i32,
    pub donations_received: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    #[serde(rename = "notMember")]
    NotMember,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Elder,
    #[serde(rename = "coLeader")]
    CoLeader,
    #[serde(rename = "leader")]
    Leader,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapital {
    pub capital_hall_level: i8,
    pub districts: Vec<District>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct District {
    pub id: i32,
    pub name: String,
    pub district_hall_level: i8,
}
