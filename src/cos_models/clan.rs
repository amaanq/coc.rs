#[cfg(feature = "cos")]
pub mod cos_clan {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Clan {
        pub tag: String,
        pub name: String,
        #[serde(rename = "type")]
        pub clan_type: String,
        pub description: String,
        pub location_id: String,
        pub badge_urls: BadgeUrls,
        pub clan_level: i32,
        pub clan_points: i32,
        pub clan_versus_points: i32,
        pub required_trophies: i32,
        pub war_frequency: String,
        pub war_win_streak: i32,
        pub war_league: i32,
        pub war_wins: i32,
        pub is_war_log_public: bool,
        pub members: i32,
        pub labels: Vec<i32>,
        pub reputation: i32,
        pub skills: Vec<Skill>,
        pub cool_down_refresh_player_end_time: i64,
        pub info: Info,
        pub has_detailed_history: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BadgeUrls {
        pub small: String,
        pub large: String,
        pub medium: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Info {}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Skill {
        pub id: i32,
        pub count: i32,
    }
}
