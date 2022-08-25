use crate::models::badge_urls::BadgeUrls;
use serde::{Deserialize, Serialize};

use super::locations::Location;

#[derive(Debug, Serialize, Deserialize)]
pub struct Clan {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "type")]
    privacy: Privacy,

    #[serde(rename = "description")]
    description: Option<String>,

    #[serde(rename = "location")]
    location: Option<Location>,

    #[serde(rename = "badgeUrls")]
    badge_urls: BadgeUrls,

    #[serde(rename = "clanLevel")]
    clan_level: i8,

    #[serde(rename = "clanPoints")]
    clan_points: i32,

    #[serde(rename = "clanVersusPoints")]
    clan_versus_points: i32,

    #[serde(rename = "requiredTrophies")]
    required_trophies: i32,

    #[serde(rename = "warFrequency")]
    war_frequency: WarFrequency,

    #[serde(rename = "warWinStreak")]
    war_win_streak: i32,

    #[serde(rename = "warWins")]
    war_wins: i32,

    #[serde(rename = "warTies")]
    war_ties: Option<i32>,

    #[serde(rename = "warLosses")]
    war_losses: Option<i32>,

    #[serde(rename = "isWarLogPublic")]
    is_war_log_public: bool,

    #[serde(rename = "warLeague")]
    war_league: WarLeague,

    #[serde(rename = "members")]
    members: i32,

    #[serde(rename = "memberList")]
    member_list: Option<Vec<ClanMember>>,

    #[serde(rename = "labels")]
    labels: Vec<Label>,

    #[serde(rename = "requiredVersusTrophies")]
    required_versus_trophies: i32,

    #[serde(rename = "requiredTownhallLevel")]
    required_townhall_level: i8,

    #[serde(rename = "clanCapital")]
    clan_capital: Option<ClanCapital>,

    #[serde(rename = "chatLanguage")]
    chat_language: Option<ChatLanguage>,
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
    // UNKNOWN, ALWAYS, MORE_THAN_ONCE_PER_WEEK, ONCE_PER_WEEK, LESS_THAN_ONCE_PER_WEEK, NEVER, ANY
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
pub struct ChatLanguage {
    #[serde(rename = "id")]
    id: i32,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "languageCode")]
    language_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    #[serde(rename = "id")]
    id: i32,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "iconUrls")]
    icon_urls: LabelIconUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelIconUrls {
    #[serde(rename = "small")]
    small: String,

    #[serde(rename = "medium")]
    medium: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClanMember {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "role")]
    role: Role,

    #[serde(rename = "expLevel")]
    exp_level: i32,

    #[serde(rename = "league")]
    league: League,

    #[serde(rename = "trophies")]
    trophies: i32,

    #[serde(rename = "versusTrophies")]
    versus_trophies: i32,

    #[serde(rename = "clanRank")]
    clan_rank: i32,

    #[serde(rename = "previousClanRank")]
    previous_clan_rank: i32,

    #[serde(rename = "donations")]
    donations: i32,

    #[serde(rename = "donationsReceived")]
    donations_received: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    #[serde(rename = "id")]
    id: i32,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "iconUrls")]
    icon_urls: LeagueIconUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueIconUrls {
    #[serde(rename = "small")]
    small: String,

    #[serde(rename = "tiny")]
    tiny: String,

    #[serde(rename = "medium")]
    medium: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarLeague {
    #[serde(rename = "id")]
    id: i32,

    #[serde(rename = "name")]
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,

    #[serde(rename = "coLeader")]
    CoLeader,

    #[serde(rename = "leader")]
    Leader,

    #[serde(rename = "member")]
    Member,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClanCapital {
    #[serde(rename = "capitalHallLevel")]
    capital_hall_level: i8,
    #[serde(rename = "districts")]
    districts: Vec<District>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "districtHallLevel")]
    district_hall_level: i8,
}

impl Clan {
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn privacy(&self) -> &Privacy {
        &self.privacy
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn location(&self) -> &Option<Location> {
        &self.location
    }
    pub fn badge_urls(&self) -> &BadgeUrls {
        &self.badge_urls
    }
    pub fn clan_level(&self) -> i8 {
        self.clan_level
    }
    pub fn clan_points(&self) -> i32 {
        self.clan_points
    }
    pub fn clan_versus_points(&self) -> i32 {
        self.clan_versus_points
    }
    pub fn required_trophies(&self) -> i32 {
        self.required_trophies
    }
    pub fn war_frequency(&self) -> &WarFrequency {
        &self.war_frequency
    }
    pub fn war_win_streak(&self) -> i32 {
        self.war_win_streak
    }
    pub fn war_wins(&self) -> i32 {
        self.war_wins
    }
    pub fn is_war_log_public(&self) -> bool {
        self.is_war_log_public
    }
    pub fn war_league(&self) -> &WarLeague {
        &self.war_league
    }
    pub fn members(&self) -> i32 {
        self.members
    }
    pub fn member_list(&self) -> &Option<Vec<ClanMember>> {
        &self.member_list
    }
    pub fn labels(&self) -> &Vec<Label> {
        &self.labels
    }
    pub fn chat_language(&self) -> &Option<ChatLanguage> {
        &self.chat_language
    }
    pub fn required_versus_trophies(&self) -> i32 {
        self.required_versus_trophies
    }
    pub fn required_townhall_level(&self) -> i8 {
        self.required_townhall_level
    }
}

impl Role {
    pub fn to_string(&self) -> &str {
        match self {
            Role::Admin => "elder",
            Role::CoLeader => "coLeader",
            Role::Leader => "leader",
            Role::Member => "member",
        }
    }
}

impl WarLeague {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl LeagueIconUrls {
    pub fn small(&self) -> &str {
        &self.small
    }
    pub fn tiny(&self) -> &str {
        &self.tiny
    }
    pub fn medium(&self) -> &Option<String> {
        &self.medium
    }
}

impl League {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn icon_urls(&self) -> &LeagueIconUrls {
        &self.icon_urls
    }
}

impl ClanMember {
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn role(&self) -> &Role {
        &self.role
    }
    pub fn exp_level(&self) -> i32 {
        self.exp_level
    }
    pub fn league(&self) -> &League {
        &self.league
    }
    pub fn trophies(&self) -> i32 {
        self.trophies
    }
    pub fn versus_trophies(&self) -> i32 {
        self.versus_trophies
    }
    pub fn clan_rank(&self) -> i32 {
        self.clan_rank
    }
    pub fn previous_clan_rank(&self) -> i32 {
        self.previous_clan_rank
    }
    pub fn donations(&self) -> i32 {
        self.donations
    }
    pub fn donations_received(&self) -> i32 {
        self.donations_received
    }
}

impl LabelIconUrls {
    pub fn small(&self) -> &str {
        &self.small
    }
    pub fn medium(&self) -> &str {
        &self.medium
    }
}

impl Label {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn icon_urls(&self) -> &LabelIconUrls {
        &self.icon_urls
    }
}

impl ChatLanguage {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn language_code(&self) -> &str {
        &self.language_code
    }
}
