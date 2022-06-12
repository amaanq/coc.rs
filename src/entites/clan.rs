#[allow(dead_code)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Clan {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "type")]
    clan_type: String,

    #[serde(rename = "description")]
    description: String,

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
    war_frequency: String,

    #[serde(rename = "warWinStreak")]
    war_win_streak: i32,

    #[serde(rename = "warWins")]
    war_wins: i32,

    #[serde(rename = "isWarLogPublic")]
    is_war_log_public: bool,

    #[serde(rename = "warLeague")]
    war_league: WarLeague,

    #[serde(rename = "members")]
    members: i32,

    #[serde(rename = "memberList")]
    member_list: Vec<MemberList>,

    #[serde(rename = "labels")]
    labels: Vec<Label>,

    #[serde(rename = "chatLanguage")]
    chat_language: ChatLanguage,

    #[serde(rename = "requiredVersusTrophies")]
    required_versus_trophies: i32,

    #[serde(rename = "requiredTownhallLevel")]
    required_townhall_level: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeUrls {
    #[serde(rename = "small")]
    small: String,

    #[serde(rename = "large")]
    large: String,

    #[serde(rename = "medium")]
    medium: String,
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
pub struct Location {
    #[serde(rename = "id")]
    id: i32,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "isCountry")]
    is_country: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberList {
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

impl Clan {
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn welcome_type(&self) -> &str {
        &self.clan_type
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn location(&self) -> &Option<Location> {
        &self.location
    }
    pub fn badge_urls(&self) -> &BadgeUrls {
        &self.badge_urls
    }
    pub fn clan_level(&self) -> i32 {
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
    pub fn war_frequency(&self) -> &str {
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
    pub fn member_list(&self) -> &Vec<MemberList> {
        &self.member_list
    }
    pub fn labels(&self) -> &Vec<Label> {
        &self.labels
    }
    pub fn chat_language(&self) -> &ChatLanguage {
        &self.chat_language
    }
    pub fn required_versus_trophies(&self) -> i32 {
        self.required_versus_trophies
    }
    pub fn required_townhall_level(&self) -> i8 {
        self.required_townhall_level
    }
}
