use serde::{Deserialize, Serialize};

use crate::models::badge_urls::BadgeUrls;

use super::{labels, leagues, location};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    /// Unfortunately this isn't returned in the /clans endpoint, only for a specific clan so I'm
    /// using an option for now
    pub clan_capital: Option<ClanCapital>,
    pub chat_language: Option<ChatLanguage>,
}

impl Clan {
    #[must_use]
    pub fn game_link(&self) -> String {
        format!(
            "https://link.clashofclans.com/en?action=OpenClanProfile&tag={}",
            self.tag.replace('#', ""),
        )
    }
    #[cfg(feature = "extra")]
    pub fn clash_of_stats_link(&self) -> String {
        format!("https://www.clashofstats.com/clans/{}/summary", self.tag.replace('#', ""))
    }
    #[cfg(feature = "extra")]
    pub fn chocolate_clash_link(&self) -> String {
        format!("https://cc.chocolateclash.com/cc_n/clan.php?tag={}", self.tag.replace('#', ""))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Copy, Ord)]
pub enum Privacy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "inviteOnly")]
    InviteOnly,
    #[serde(rename = "closed")]
    Closed,
}

impl Privacy {
    #[must_use]
    pub fn is_open(&self) -> bool {
        self == &Self::Open
    }
    #[must_use]
    pub fn is_invite_only(&self) -> bool {
        self == &Self::InviteOnly
    }
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self == &Self::Closed
    }
}

impl ToString for Privacy {
    fn to_string(&self) -> String {
        match self {
            Self::Open => "Anyone Can Join".to_string(),
            Self::InviteOnly => "Invite Only".to_string(),
            Self::Closed => "Closed".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Copy, Ord)]
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

impl WarFrequency {
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self == &Self::Unknown
    }
    #[must_use]
    pub fn is_always(&self) -> bool {
        self == &Self::Always
    }
    #[must_use]
    pub fn is_more_than_once_per_week(&self) -> bool {
        self == &Self::MoreThanOncePerWeek
    }
    #[must_use]
    pub fn is_once_per_week(&self) -> bool {
        self == &Self::OncePerWeek
    }
    #[must_use]
    pub fn is_less_than_once_per_week(&self) -> bool {
        self == &Self::LessThanOncePerWeek
    }
    #[must_use]
    pub fn is_never(&self) -> bool {
        self == &Self::Never
    }
    #[must_use]
    pub fn is_any(&self) -> bool {
        self == &Self::Any
    }
}

impl ToString for WarFrequency {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => "Not set".to_string(),
            Self::Always => "Always".to_string(),
            Self::MoreThanOncePerWeek => "Twice a week".to_string(),
            Self::OncePerWeek => "Once a week".to_string(),
            Self::LessThanOncePerWeek => "Rarely".to_string(),
            Self::Never => "Never".to_string(),
            Self::Any => "Any".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatLanguage {
    pub id: i32,
    pub name: String,
    pub language_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

impl Role {
    #[must_use]
    pub fn is_not_member(&self) -> bool {
        self == &Self::NotMember
    }
    #[must_use]
    pub fn is_member(&self) -> bool {
        self == &Self::Member
    }
    #[must_use]
    pub fn is_elder(&self) -> bool {
        self == &Self::Elder
    }
    #[must_use]
    pub fn is_co_leader(&self) -> bool {
        self == &Self::CoLeader
    }
    #[must_use]
    pub fn is_leader(&self) -> bool {
        self == &Self::Leader
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Self::NotMember => "Not Member".to_string(),
            Self::Member => "Member".to_string(),
            Self::Elder => "Elder".to_string(),
            Self::CoLeader => "Co-Leader".to_string(),
            Self::Leader => "Leader".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapital {
    capital_hall_level: Option<i8>,
    districts: Option<Vec<District>>,
}

impl ClanCapital {
    #[must_use]
    pub fn capital_hall_level(&self) -> i8 {
        self.capital_hall_level.unwrap_or_default()
    }
    #[must_use]
    pub fn districts(&self) -> Vec<District> {
        self.districts.clone().unwrap_or_default()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct District {
    pub id: i32,
    pub name: String,
    pub district_hall_level: i8,
}
