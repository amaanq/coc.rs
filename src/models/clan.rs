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

impl Clan {
    pub fn game_link(&self) -> String {
        format!(
            "https://link.clashofclans.com/en?action=OpenClanProfile&tag={}",
            self.tag.replace("#", ""),
        )
    }

    pub fn clash_of_stats_link(&self) -> String {
        format!(
            "https://www.clashofstats.com/clans/{}/summary",
            self.tag.replace("#", "")
        )
    }

    pub fn chocolate_clash_link(&self) -> String {
        format!(
            "https://cc.chocolateclash.com/cc_n/clan.php?tag={}",
            self.tag.replace("#", "")
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Privacy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "inviteOnly")]
    InviteOnly,
    #[serde(rename = "closed")]
    Closed,
}

impl Privacy {
    pub fn is_open(&self) -> bool {
        self == &Privacy::Open
    }
    pub fn is_invite_only(&self) -> bool {
        self == &Privacy::InviteOnly
    }
    pub fn is_closed(&self) -> bool {
        self == &Privacy::Closed
    }
}

impl ToString for Privacy {
    fn to_string(&self) -> String {
        match self {
            Privacy::Open => "Anyone Can Join".to_string(),
            Privacy::InviteOnly => "Invite Only".to_string(),
            Privacy::Closed => "Closed".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn is_unknown(&self) -> bool {
        self == &WarFrequency::Unknown
    }
    pub fn is_always(&self) -> bool {
        self == &WarFrequency::Always
    }
    pub fn is_more_than_once_per_week(&self) -> bool {
        self == &WarFrequency::MoreThanOncePerWeek
    }
    pub fn is_once_per_week(&self) -> bool {
        self == &WarFrequency::OncePerWeek
    }
    pub fn is_less_than_once_per_week(&self) -> bool {
        self == &WarFrequency::LessThanOncePerWeek
    }
    pub fn is_never(&self) -> bool {
        self == &WarFrequency::Never
    }
    pub fn is_any(&self) -> bool {
        self == &WarFrequency::Any
    }
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

impl Role {
    pub fn is_not_member(&self) -> bool {
        self == &Role::NotMember
    }
    pub fn is_member(&self) -> bool {
        self == &Role::Member
    }
    pub fn is_elder(&self) -> bool {
        self == &Role::Elder
    }
    pub fn is_co_leader(&self) -> bool {
        self == &Role::CoLeader
    }
    pub fn is_leader(&self) -> bool {
        self == &Role::Leader
    }
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
