#[cfg(feature = "cos")]
pub mod leaderboard {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClanLeaderboard {
        pub size: i32,
        pub rankings: Vec<crate::ClanRanking>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlayerLeaderboard {
        pub size: i32,
        pub rankings: Vec<crate::PlayerRanking>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LegendsLeaderboard {
        pub size: i32,
        pub rankings: Vec<crate::LegendsRanking>,
    }
}
