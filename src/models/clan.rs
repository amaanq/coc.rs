use crate::models::badge_urls::BadgeUrls;
use serde::{Deserialize, Serialize};

use super::locations::Location;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub tag: String,
    pub name: String,
    #[serde(rename = "type")]
    pub privacy: Privacy,
    pub description: Option<String>,
    pub location: Option<Location>,
    pub badge_urls: BadgeUrls,
    pub clan_level: i8,
    pub clan_points: i32,
    pub clan_versus_points: i32,
    pub required_trophies: i32,
    pub war_frequency: WarFrequency,
    pub war_win_streak: i32,
    pub war_wins: i32,
    pub war_ties: Option<i32>,
    pub war_losses: Option<i32>,
    pub is_war_log_public: bool,
    pub war_league: WarLeague,
    pub members: i32,
    pub member_list: Option<Vec<ClanMember>>,
    pub labels: Vec<Label>,
    pub required_versus_trophies: i32,
    pub required_townhall_level: i8,
    pub clan_capital: Option<ClanCapital>,
    pub chat_language: Option<ChatLanguage>,
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
#[serde(rename_all = "camelCase")]
pub struct ChatLanguage {
    pub id: i32,
    pub name: String,
    pub language_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: i32,
    pub name: String,
    pub icon_urls: LabelIconUrls,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelIconUrls {
    pub small: String,
    pub medium: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanMember {
    pub tag: String,
    pub name: String,
    pub role: Role,
    pub exp_level: i32,
    pub league: League,
    pub trophies: i32,
    pub versus_trophies: i32,
    pub clan_rank: i32,
    pub previous_clan_rank: i32,
    pub donations: i32,
    pub donations_received: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    #[serde(rename = "notMember")]
    NotMember,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Elder,
    #[serde(rename = "coLeader")]
    CoLeader,
    #[serde(rename = "leader")]
    Leader,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: i32,
    pub name: String,
    pub icon_urls: LeagueIconUrls,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueIconUrls {
    pub small: String,
    pub tiny: String,
    pub medium: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarLeague {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapital {
    pub capital_hall_level: i8,
    pub districts: Vec<District>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct District {
    pub id: i32,
    pub name: String,
    pub district_hall_level: i8,
}

impl League {
    pub fn Unranked() -> Self {
        League {
            id: 29000000,
            name: "Unranked".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/e--YMyIexEQQhE4imLoJcwhYn6Uy8KqlgyY3_kFV6t4.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/e--YMyIexEQQhE4imLoJcwhYn6Uy8KqlgyY3_kFV6t4.png".to_string(),
                medium: None,
            },
        }
    }

    pub fn BronzeLeagueIII() -> Self {
        League {
            id: 29000001,
            name: "Bronze League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/uUJDLEdAh7Lwf6YOHmXfNM586ZlEvMju54bTlt2u6EE.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/uUJDLEdAh7Lwf6YOHmXfNM586ZlEvMju54bTlt2u6EE.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/uUJDLEdAh7Lwf6YOHmXfNM586ZlEvMju54bTlt2u6EE.png".to_string()),
            },
        }
    }

    pub fn BronzeLeagueII() -> Self {
        League {
            id: 29000002,
            name: "Bronze League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/U2acNDRaR5rQDu4Z6pQKaGcjWm9dkSnHMAPZCXrHPB4.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/U2acNDRaR5rQDu4Z6pQKaGcjWm9dkSnHMAPZCXrHPB4.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/U2acNDRaR5rQDu4Z6pQKaGcjWm9dkSnHMAPZCXrHPB4.png".to_string()),
            },
        }
    }

    pub fn BronzeLeagueI() -> Self {
        League {
            id: 29000003,
            name: "Bronze League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/SZIXZHZxfHTmgseKCH6T5hvMQ3JQM-Js2QfpC9A3ya0.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/SZIXZHZxfHTmgseKCH6T5hvMQ3JQM-Js2QfpC9A3ya0.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/SZIXZHZxfHTmgseKCH6T5hvMQ3JQM-Js2QfpC9A3ya0.png".to_string()),
            },
        }
    }

    pub fn SilverLeagueIII() -> Self {
        League {
            id: 29000004,
            name: "Silver League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/QcFBfoArnafaXCnB5OfI7vESpQEBuvWtzOyLq8gJzVc.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/QcFBfoArnafaXCnB5OfI7vESpQEBuvWtzOyLq8gJzVc.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/QcFBfoArnafaXCnB5OfI7vESpQEBuvWtzOyLq8gJzVc.png".to_string()),
            },
        }
    }

    pub fn SilverLeagueII() -> Self {
        League {
            id: 29000005,
            name: "Silver League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/8OhXcwDJkenBH2kPH73eXftFOpHHRF-b32n0yrTqC44.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/8OhXcwDJkenBH2kPH73eXftFOpHHRF-b32n0yrTqC44.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/8OhXcwDJkenBH2kPH73eXftFOpHHRF-b32n0yrTqC44.png".to_string()),
            },
        }
    }

    pub fn SilverLeagueI() -> Self {
        League {
            id: 29000006,
            name: "Silver League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/nvrBLvCK10elRHmD1g9w5UU1flDRMhYAojMB2UbYfPs.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/nvrBLvCK10elRHmD1g9w5UU1flDRMhYAojMB2UbYfPs.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/nvrBLvCK10elRHmD1g9w5UU1flDRMhYAojMB2UbYfPs.png".to_string()),
            },
        }
    }

    pub fn GoldLeagueIII() -> Self {
        League {
            id: 29000007,
            name: "Gold League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/vd4Lhz5b2I1P0cLH25B6q63JN3Wt1j2NTMhOYpMPQ4M.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/vd4Lhz5b2I1P0cLH25B6q63JN3Wt1j2NTMhOYpMPQ4M.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/vd4Lhz5b2I1P0cLH25B6q63JN3Wt1j2NTMhOYpMPQ4M.png".to_string()),
            },
        }
    }

    pub fn GoldLeagueII() -> Self {
        League {
            id: 29000008,
            name: "Gold League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/Y6CveuHmPM_oiOic2Yet0rYL9AFRYW0WA0u2e44-YbM.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/Y6CveuHmPM_oiOic2Yet0rYL9AFRYW0WA0u2e44-YbM.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/Y6CveuHmPM_oiOic2Yet0rYL9AFRYW0WA0u2e44-YbM.png".to_string()),
            },
        }
    }

    pub fn GoldLeagueI() -> Self {
        League {
            id: 29000009,
            name: "Gold League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/CorhMY9ZmQvqXTZ4VYVuUgPNGSHsO0cEXEL5WYRmB2Y.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/CorhMY9ZmQvqXTZ4VYVuUgPNGSHsO0cEXEL5WYRmB2Y.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/CorhMY9ZmQvqXTZ4VYVuUgPNGSHsO0cEXEL5WYRmB2Y.png".to_string()),
            },
        }
    }

    pub fn CrystalLeagueIII() -> Self {
        League {
            id: 29000010,
            name: "Crystal League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/Hyqco7bHh0Q81xB8mSF_ZhjKnKcTmJ9QEq9QGlsxiKE.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/Hyqco7bHh0Q81xB8mSF_ZhjKnKcTmJ9QEq9QGlsxiKE.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/Hyqco7bHh0Q81xB8mSF_ZhjKnKcTmJ9QEq9QGlsxiKE.png".to_string()),
            },
        }
    }

    pub fn CrystalLeagueII() -> Self {
        League {
            id: 29000011,
            name: "Crystal League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/jhP36EhAA9n1ADafdQtCP-ztEAQjoRpY7cT8sU7SW8A.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/jhP36EhAA9n1ADafdQtCP-ztEAQjoRpY7cT8sU7SW8A.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/jhP36EhAA9n1ADafdQtCP-ztEAQjoRpY7cT8sU7SW8A.png".to_string()),
            },
        }
    }

    pub fn CrystalLeagueI() -> Self {
        League {
            id: 29000012,
            name: "Crystal League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/kSfTyNNVSvogX3dMvpFUTt72VW74w6vEsEFuuOV4osQ.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/kSfTyNNVSvogX3dMvpFUTt72VW74w6vEsEFuuOV4osQ.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/kSfTyNNVSvogX3dMvpFUTt72VW74w6vEsEFuuOV4osQ.png".to_string()),
            },
        }
    }

    pub fn MasterLeagueIII() -> Self {
        League {
            id: 29000013,
            name: "Master League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/pSXfKvBKSgtvfOY3xKkgFaRQi0WcE28s3X35ywbIluY.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/pSXfKvBKSgtvfOY3xKkgFaRQi0WcE28s3X35ywbIluY.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/pSXfKvBKSgtvfOY3xKkgFaRQi0WcE28s3X35ywbIluY.png".to_string()),
            },
        }
    }

    pub fn MasterLeagueII() -> Self {
        League {
            id: 29000014,
            name: "Master League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/4wtS1stWZQ-1VJ5HaCuDPfdhTWjeZs_jPar_YPzK6Lg.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/4wtS1stWZQ-1VJ5HaCuDPfdhTWjeZs_jPar_YPzK6Lg.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/4wtS1stWZQ-1VJ5HaCuDPfdhTWjeZs_jPar_YPzK6Lg.png".to_string()),
            },
        }
    }

    pub fn MasterLeagueI() -> Self {
        League {
            id: 29000015,
            name: "Master League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/olUfFb1wscIH8hqECAdWbdB6jPm9R8zzEyHIzyBgRXc.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/olUfFb1wscIH8hqECAdWbdB6jPm9R8zzEyHIzyBgRXc.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/olUfFb1wscIH8hqECAdWbdB6jPm9R8zzEyHIzyBgRXc.png".to_string()),
            },
        }
    }

    pub fn ChampionLeagueIII() -> Self {
        League {
            id: 29000016,
            name: "Champion League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/JmmTbspV86xBigM7OP5_SjsEDPuE7oXjZC9aOy8xO3s.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/JmmTbspV86xBigM7OP5_SjsEDPuE7oXjZC9aOy8xO3s.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/JmmTbspV86xBigM7OP5_SjsEDPuE7oXjZC9aOy8xO3s.png".to_string()),
            },
        }
    }

    pub fn ChampionLeagueII() -> Self {
        League {
            id: 29000017,
            name: "Champion League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/kLWSSyq7vJiRiCantiKCoFuSJOxief6R1ky6AyfB8q0.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/kLWSSyq7vJiRiCantiKCoFuSJOxief6R1ky6AyfB8q0.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/kLWSSyq7vJiRiCantiKCoFuSJOxief6R1ky6AyfB8q0.png".to_string()),
            },
        }
    }

    pub fn ChampionLeagueI() -> Self {
        League {
            id: 29000018,
            name: "Champion League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/9v_04LHmd1LWq7IoY45dAdGhrBkrc2ZFMZVhe23PdCE.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/9v_04LHmd1LWq7IoY45dAdGhrBkrc2ZFMZVhe23PdCE.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/9v_04LHmd1LWq7IoY45dAdGhrBkrc2ZFMZVhe23PdCE.png".to_string()),
            },
        }
    }

    pub fn TitanLeagueIII() -> Self {
        League {
            id: 29000019,
            name: "Titan League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/L-HrwYpFbDwWjdmhJQiZiTRa_zXPPOgUTdmbsaq4meo.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/L-HrwYpFbDwWjdmhJQiZiTRa_zXPPOgUTdmbsaq4meo.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/L-HrwYpFbDwWjdmhJQiZiTRa_zXPPOgUTdmbsaq4meo.png".to_string()),
            },
        }
    }

    pub fn TitanLeagueII() -> Self {
        League {
            id: 29000020,
            name: "Titan League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/llpWocHlOoFliwyaEx5Z6dmoZG4u4NmxwpF-Jg7su7Q.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/llpWocHlOoFliwyaEx5Z6dmoZG4u4NmxwpF-Jg7su7Q.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/llpWocHlOoFliwyaEx5Z6dmoZG4u4NmxwpF-Jg7su7Q.png".to_string()),
            },
        }
    }

    pub fn TitanLeagueI() -> Self {
        League {
            id: 29000021,
            name: "Titan League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png".to_string()),
            },
        }
    }

    pub fn LegendLeague() -> Self {
        League {
            id: 29000022,
            name: "Legend League".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/R2zmhyqQ0_lKcDR5EyghXCxgyC9mm_mVMIjAbmGoZtw.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/R2zmhyqQ0_lKcDR5EyghXCxgyC9mm_mVMIjAbmGoZtw.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/R2zmhyqQ0_lKcDR5EyghXCxgyC9mm_mVMIjAbmGoZtw.png".to_string()),
            },
        }
    }
}
