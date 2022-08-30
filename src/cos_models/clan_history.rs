#[cfg(feature = "cos")]
pub mod cos_clan_history {
    use serde::{Deserialize, Serialize};

    use crate::clan::Role;

    pub type ClanPastMembers = Vec<PastMember>;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PastMember {
        pub tag: String,
        pub role: Option<Role>,
        pub first_seen: i64,
        pub end_date: i64,
        pub in_clan: bool,
        pub character_id: Option<String>,
        pub name: Option<String>,
        pub town_hall_level: Option<i8>,
        pub town_hall_weapon_level: Option<i8>,
        pub builder_hall_level: Option<i8>,
        pub is_vip: Option<bool>,
    }
}
