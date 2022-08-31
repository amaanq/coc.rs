#[cfg(feature = "cos")]
pub mod cos_player_history {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    use crate::clan::Role;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerHistory {
        pub log: Vec<Log>,
        pub summary: Vec<Summary>,
        pub roles_in_clans: Vec<i32>,
        pub clans_map: HashMap<String, ClansMap>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClansMap {
        pub name: String,
        pub tag: String,
        pub badge: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Log {
        #[serde(rename = "type")]
        pub log_type: Type,
        pub tag: Option<String>,
        pub role: Option<Role>,
        pub start: Option<String>,
        pub end: Option<String>,
        pub duration: i64,
        pub date: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Summary {
        pub tag: String,
        pub duration: i64,
        pub count: i32,
        pub roles: Vec<i32>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "SEEN")]
        Seen,
        #[serde(rename = "STAY")]
        Stay,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }
}
