use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct War {
    #[serde(rename = "state")]
    state: String,

    #[serde(rename = "teamSize")]
    team_size: i32,

    #[serde(rename = "attacksPerMember")]
    attacks_per_member: i8,

    #[serde(rename = "preparationStartTime")]
    preparation_start_time: String,

    #[serde(rename = "startTime")]
    start_time: String,

    #[serde(rename = "endTime")]
    end_time: String,

    #[serde(rename = "clan")]
    clan: WarClan,

    #[serde(rename = "opponent")]
    opponent: WarClan,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarClan {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "badgeUrls")]
    badge_urls: BadgeUrls,

    #[serde(rename = "clanLevel")]
    clan_level: i8,

    #[serde(rename = "attacks")]
    attacks: i32,

    #[serde(rename = "stars")]
    stars: i32,

    #[serde(rename = "destructionPercentage")]
    destruction_percentage: f64,

    #[serde(rename = "members")]
    members: Vec<Member>,
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
pub struct Member {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "townhallLevel")]
    townhall_level: i8,

    #[serde(rename = "mapPosition")]
    map_position: i32,

    #[serde(rename = "attacks")]
    attacks: Vec<Attack>,

    #[serde(rename = "opponentAttacks")]
    opponent_attacks: i32,

    #[serde(rename = "bestOpponentAttack")]
    best_opponent_attack: Attack,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attack {
    #[serde(rename = "attackerTag")]
    attacker_tag: String,

    #[serde(rename = "defenderTag")]
    defender_tag: String,

    #[serde(rename = "stars")]
    stars: i32,

    #[serde(rename = "destructionPercentage")]
    destruction_percentage: f32,

    #[serde(rename = "order")]
    order: i32,

    #[serde(rename = "duration")]
    duration: i32,
}
