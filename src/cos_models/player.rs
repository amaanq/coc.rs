#[cfg(feature = "cos")]
pub mod cos_player {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Player {
        pub clan: Clan,
        pub tag: String,
        pub name: String,
        pub character_id: String,
        pub town_hall_level: i32,
        pub town_hall_weapon_level: Option<i32>,
        pub builder_hall_level: Option<i32>,
        pub exp_level: i32,
        pub trophies: i32,
        pub trophy_league_id: i32,
        pub legend_trophies: i32,
        pub war_stars: i32,
        pub versus_trophies: i32,
        pub versus_battle_wins: i32,
        pub location_id: String,
        pub official_location: Option<String>,
        pub donations: i32,
        pub donations_received: i32,
        pub attack_wins: i32,
        pub defense_wins: i32,
        pub best_trophies: i32,
        pub best_versus_trophies: i32,
        pub info: Info,
        pub languages: Vec<Option<serde_json::Value>>,
        pub labels: Vec<i32>,
        pub reputation: i32,
        pub is_vip: bool,
        pub skills: Vec<Skill>,
        pub player_history_start_date: String,
        pub achievements: Vec<Achievement>,
        pub troops: Vec<Hero>,
        pub spells: Vec<Hero>,
        pub heroes: Vec<Hero>,
        pub super_troops: Vec<SuperTroop>,
        pub date_added: String,
        pub best_donations: BestDonations,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Achievement {
        pub id: i32,
        pub stars: i32,
        pub value: i32,
        pub target: i32,
        pub order: i32,
        pub village: Village,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BestDonations {
        pub val: i32,
        pub date: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Clan {
        pub tag: String,
        pub name: String,
        pub badge: String,
        pub role: String,
        pub rank: i32,
        pub previous_rank: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Hero {
        pub id: i32,
        pub order: i32,
        pub level: i32,
        pub max_level_for_player: i32,
        pub max_level: i32,
        pub village: Village,
        pub is_dark: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Info {}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Skill {
        pub id: i32,
        pub count: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SuperTroop {
        pub id: i32,
        pub api_name: String,
        pub wiki_path: String,
        pub original_troop_id: i32,
        pub min_original_level: i32,
        pub order: i32,
        pub level: i32,
        pub max_level: i32,
        pub max_level_for_player: i32,
        pub is_unlocked: bool,
        pub is_active: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Village {
        #[serde(rename = "builder")]
        BuilderBase,
        #[serde(rename = "home")]
        Home,
    }
}
