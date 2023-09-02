use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::icon_urls;

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(i32)]
pub enum PlayerLabelKind {
    ClanWars = 57_000_000,
    ClanWarLeague = 57_000_001,
    TrophyPushing = 57_000_002,
    FriendlyWars = 57_000_003,
    ClanGames = 57_000_004,
    BuilderBase = 57_000_005,
    BaseDesigning = 57_000_006,
    Farming = 57_000_007,
    ActiveDonator = 57_000_008,
    ActiveDaily = 57_000_009,
    HungryLearner = 57_000_010,
    Friendly = 57_000_011,
    Talkative = 57_000_012,
    Teacher = 57_000_013,
    Competitive = 57_000_014,
    Veteran = 57_000_015,
    Newbie = 57_000_016,
    AmateurAttacker = 57_000_017,
    ClanCapital = 57_000_018,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(i32)]
pub enum ClanLabelKind {
    ClanWars = 56_000_000,
    ClanWarLeague = 56_000_001,
    TrophyPushing = 56_000_002,
    FriendlyWars = 56_000_003,
    ClanGames = 56_000_004,
    BuilderBase = 56_000_005,
    BaseDesigning = 56_000_006,
    International = 56_000_007,
    Farming = 56_000_008,
    Donations = 56_000_009,
    Friendly = 56_000_010,
    Talkative = 56_000_011,
    Underdog = 56_000_012,
    Relaxed = 56_000_013,
    Competitive = 56_000_014,
    NewbieFriendly = 56_000_015,
    ClanCapital = 56_000_016,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLabel {
    pub id: PlayerLabelKind,
    pub name: String,
    pub icon_urls: icon_urls::LabelIconUrls,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClanLabel {
    pub id: ClanLabelKind,
    pub name: String,
    pub icon_urls: icon_urls::LabelIconUrls,
}
