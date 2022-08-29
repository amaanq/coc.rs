use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::*;

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum PlayerLabelKind {
    ClanWars = 57000000,
    ClanWarLeague = 57000001,
    TrophyPushing = 57000002,
    FriendlyWars = 57000003,
    ClanGames = 57000004,
    BuilderBase = 57000005,
    BaseDesigning = 57000006,
    Farming = 57000007,
    ActiveDonator = 57000008,
    ActiveDaily = 57000009,
    HungryLearner = 57000010,
    Friendly = 57000011,
    Talkative = 57000012,
    Teacher = 57000013,
    Competitive = 57000014,
    Veteran = 57000015,
    Newbie = 57000016,
    AmateurAttacker = 57000017,
    ClanCapital = 57000018,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClanLabelKind {
    ClanWars = 56000000,
    ClanWarLeague = 56000001,
    TrophyPushing = 56000002,
    FriendlyWars = 56000003,
    ClanGames = 56000004,
    BuilderBase = 56000005,
    BaseDesigning = 56000006,
    International = 56000007,
    Farming = 56000008,
    Donations = 56000009,
    Friendly = 56000010,
    Talkative = 56000011,
    Underdog = 56000012,
    Relaxed = 56000013,
    Competitive = 56000014,
    NewbieFriendly = 56000015,
    ClanCapital = 56000016,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLabel {
    pub id: PlayerLabelKind,
    pub name: String,
    pub icon_urls: icon_urls::LabelIconUrls,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanLabel {
    pub id: ClanLabelKind,
    pub name: String,
    pub icon_urls: icon_urls::LabelIconUrls,
}
