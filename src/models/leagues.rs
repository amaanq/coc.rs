use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::icon_urls::LeagueIconUrls;

use super::icon_urls;

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Clone, Ord)]
#[repr(i32)]
pub enum LeagueKind {
    Unranked = 29_000_000,
    BronzeLeagueIII = 29_000_001,
    BronzeLeagueII = 29_000_002,
    BronzeLeagueI = 29_000_003,
    SilverLeagueIII = 29_000_004,
    SilverLeagueII = 29_000_005,
    SilverLeagueI = 29_000_006,
    GoldLeagueIII = 29_000_007,
    GoldLeagueII = 29_000_008,
    GoldLeagueI = 29_000_009,
    CrystalLeagueIII = 29_000_010,
    CrystalLeagueII = 29_000_011,
    CrystalLeagueI = 29_000_012,
    MasterLeagueIII = 29_000_013,
    MasterLeagueII = 29_000_014,
    MasterLeagueI = 29_000_015,
    ChampionLeagueIII = 29_000_016,
    ChampionLeagueII = 29_000_017,
    ChampionLeagueI = 29_000_018,
    TitanLeagueIII = 29_000_019,
    TitanLeagueII = 29_000_020,
    TitanLeagueI = 29_000_021,
    LegendLeague = 29_000_022,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Clone, Ord)]
#[repr(i32)]
pub enum WarLeagueKind {
    Unranked = 48_000_000,
    BronzeLeagueIII = 48_000_001,
    BronzeLeagueII = 48_000_002,
    BronzeLeagueI = 48_000_003,
    SilverLeagueIII = 48_000_004,
    SilverLeagueII = 48_000_005,
    SilverLeagueI = 48_000_006,
    GoldLeagueIII = 48_000_007,
    GoldLeagueII = 48_000_008,
    GoldLeagueI = 48_000_009,
    CrystalLeagueIII = 48_000_010,
    CrystalLeagueII = 48_000_011,
    CrystalLeagueI = 48_000_012,
    MasterLeagueIII = 48_000_013,
    MasterLeagueII = 48_000_014,
    MasterLeagueI = 48_000_015,
    ChampionLeagueIII = 48_000_016,
    ChampionLeagueII = 48_000_017,
    ChampionLeagueI = 48_000_018,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: LeagueKind,
    pub name: String,
    pub icon_urls: icon_urls::LeagueIconUrls,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WarLeague {
    pub id: WarLeagueKind,
    pub name: String,
}

#[allow(non_snake_case)]
/// I need to think of a better way to do this, enum variants with struct types seem stupid as they're all the same type.
impl League {
    #[must_use]
    pub fn Unranked() -> Self {
        Self {
            id: LeagueKind::Unranked,
            name: "Unranked".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/e--YMyIexEQQhE4imLoJcwhYn6Uy8KqlgyY3_kFV6t4.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/e--YMyIexEQQhE4imLoJcwhYn6Uy8KqlgyY3_kFV6t4.png".to_string(),
                medium: None,
            },
        }
    }

    #[must_use]
    pub fn BronzeLeagueIII() -> Self {
        Self {
            id: LeagueKind::BronzeLeagueIII,
            name: "Bronze League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/uUJDLEdAh7Lwf6YOHmXfNM586ZlEvMju54bTlt2u6EE.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/uUJDLEdAh7Lwf6YOHmXfNM586ZlEvMju54bTlt2u6EE.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/uUJDLEdAh7Lwf6YOHmXfNM586ZlEvMju54bTlt2u6EE.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn BronzeLeagueII() -> Self {
        Self {
            id: LeagueKind::BronzeLeagueII,
            name: "Bronze League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/U2acNDRaR5rQDu4Z6pQKaGcjWm9dkSnHMAPZCXrHPB4.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/U2acNDRaR5rQDu4Z6pQKaGcjWm9dkSnHMAPZCXrHPB4.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/U2acNDRaR5rQDu4Z6pQKaGcjWm9dkSnHMAPZCXrHPB4.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn BronzeLeagueI() -> Self {
        Self {
            id: LeagueKind::BronzeLeagueI,
            name: "Bronze League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/SZIXZHZxfHTmgseKCH6T5hvMQ3JQM-Js2QfpC9A3ya0.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/SZIXZHZxfHTmgseKCH6T5hvMQ3JQM-Js2QfpC9A3ya0.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/SZIXZHZxfHTmgseKCH6T5hvMQ3JQM-Js2QfpC9A3ya0.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn SilverLeagueIII() -> Self {
        Self {
            id: LeagueKind::SilverLeagueIII,
            name: "Silver League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/QcFBfoArnafaXCnB5OfI7vESpQEBuvWtzOyLq8gJzVc.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/QcFBfoArnafaXCnB5OfI7vESpQEBuvWtzOyLq8gJzVc.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/QcFBfoArnafaXCnB5OfI7vESpQEBuvWtzOyLq8gJzVc.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn SilverLeagueII() -> Self {
        Self {
            id: LeagueKind::SilverLeagueII,
            name: "Silver League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/8OhXcwDJkenBH2kPH73eXftFOpHHRF-b32n0yrTqC44.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/8OhXcwDJkenBH2kPH73eXftFOpHHRF-b32n0yrTqC44.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/8OhXcwDJkenBH2kPH73eXftFOpHHRF-b32n0yrTqC44.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn SilverLeagueI() -> Self {
        Self {
            id: LeagueKind::SilverLeagueI,
            name: "Silver League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/nvrBLvCK10elRHmD1g9w5UU1flDRMhYAojMB2UbYfPs.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/nvrBLvCK10elRHmD1g9w5UU1flDRMhYAojMB2UbYfPs.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/nvrBLvCK10elRHmD1g9w5UU1flDRMhYAojMB2UbYfPs.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn GoldLeagueIII() -> Self {
        Self {
            id: LeagueKind::GoldLeagueIII,
            name: "Gold League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/vd4Lhz5b2I1P0cLH25B6q63JN3Wt1j2NTMhOYpMPQ4M.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/vd4Lhz5b2I1P0cLH25B6q63JN3Wt1j2NTMhOYpMPQ4M.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/vd4Lhz5b2I1P0cLH25B6q63JN3Wt1j2NTMhOYpMPQ4M.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn GoldLeagueII() -> Self {
        Self {
            id: LeagueKind::GoldLeagueII,
            name: "Gold League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/Y6CveuHmPM_oiOic2Yet0rYL9AFRYW0WA0u2e44-YbM.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/Y6CveuHmPM_oiOic2Yet0rYL9AFRYW0WA0u2e44-YbM.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/Y6CveuHmPM_oiOic2Yet0rYL9AFRYW0WA0u2e44-YbM.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn GoldLeagueI() -> Self {
        Self {
            id: LeagueKind::GoldLeagueI,
            name: "Gold League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/CorhMY9ZmQvqXTZ4VYVuUgPNGSHsO0cEXEL5WYRmB2Y.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/CorhMY9ZmQvqXTZ4VYVuUgPNGSHsO0cEXEL5WYRmB2Y.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/CorhMY9ZmQvqXTZ4VYVuUgPNGSHsO0cEXEL5WYRmB2Y.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn CrystalLeagueIII() -> Self {
        Self {
            id: LeagueKind::CrystalLeagueIII,
            name: "Crystal League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/Hyqco7bHh0Q81xB8mSF_ZhjKnKcTmJ9QEq9QGlsxiKE.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/Hyqco7bHh0Q81xB8mSF_ZhjKnKcTmJ9QEq9QGlsxiKE.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/Hyqco7bHh0Q81xB8mSF_ZhjKnKcTmJ9QEq9QGlsxiKE.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn CrystalLeagueII() -> Self {
        Self {
            id: LeagueKind::CrystalLeagueII,
            name: "Crystal League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/jhP36EhAA9n1ADafdQtCP-ztEAQjoRpY7cT8sU7SW8A.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/jhP36EhAA9n1ADafdQtCP-ztEAQjoRpY7cT8sU7SW8A.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/jhP36EhAA9n1ADafdQtCP-ztEAQjoRpY7cT8sU7SW8A.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn CrystalLeagueI() -> Self {
        Self {
            id: LeagueKind::CrystalLeagueI,
            name: "Crystal League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/kSfTyNNVSvogX3dMvpFUTt72VW74w6vEsEFuuOV4osQ.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/kSfTyNNVSvogX3dMvpFUTt72VW74w6vEsEFuuOV4osQ.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/kSfTyNNVSvogX3dMvpFUTt72VW74w6vEsEFuuOV4osQ.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn MasterLeagueIII() -> Self {
        Self {
            id: LeagueKind::MasterLeagueIII,
            name: "Master League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/pSXfKvBKSgtvfOY3xKkgFaRQi0WcE28s3X35ywbIluY.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/pSXfKvBKSgtvfOY3xKkgFaRQi0WcE28s3X35ywbIluY.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/pSXfKvBKSgtvfOY3xKkgFaRQi0WcE28s3X35ywbIluY.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn MasterLeagueII() -> Self {
        Self {
            id: LeagueKind::MasterLeagueII,
            name: "Master League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/4wtS1stWZQ-1VJ5HaCuDPfdhTWjeZs_jPar_YPzK6Lg.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/4wtS1stWZQ-1VJ5HaCuDPfdhTWjeZs_jPar_YPzK6Lg.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/4wtS1stWZQ-1VJ5HaCuDPfdhTWjeZs_jPar_YPzK6Lg.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn MasterLeagueI() -> Self {
        Self {
            id: LeagueKind::MasterLeagueI,
            name: "Master League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/olUfFb1wscIH8hqECAdWbdB6jPm9R8zzEyHIzyBgRXc.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/olUfFb1wscIH8hqECAdWbdB6jPm9R8zzEyHIzyBgRXc.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/olUfFb1wscIH8hqECAdWbdB6jPm9R8zzEyHIzyBgRXc.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn ChampionLeagueIII() -> Self {
        Self {
            id: LeagueKind::ChampionLeagueIII,
            name: "Champion League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/JmmTbspV86xBigM7OP5_SjsEDPuE7oXjZC9aOy8xO3s.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/JmmTbspV86xBigM7OP5_SjsEDPuE7oXjZC9aOy8xO3s.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/JmmTbspV86xBigM7OP5_SjsEDPuE7oXjZC9aOy8xO3s.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn ChampionLeagueII() -> Self {
        Self {
            id: LeagueKind::ChampionLeagueII,
            name: "Champion League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/kLWSSyq7vJiRiCantiKCoFuSJOxief6R1ky6AyfB8q0.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/kLWSSyq7vJiRiCantiKCoFuSJOxief6R1ky6AyfB8q0.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/kLWSSyq7vJiRiCantiKCoFuSJOxief6R1ky6AyfB8q0.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn ChampionLeagueI() -> Self {
        Self {
            id: LeagueKind::ChampionLeagueI,
            name: "Champion League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/9v_04LHmd1LWq7IoY45dAdGhrBkrc2ZFMZVhe23PdCE.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/9v_04LHmd1LWq7IoY45dAdGhrBkrc2ZFMZVhe23PdCE.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/9v_04LHmd1LWq7IoY45dAdGhrBkrc2ZFMZVhe23PdCE.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn TitanLeagueIII() -> Self {
        Self {
            id: LeagueKind::TitanLeagueIII,
            name: "Titan League III".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/L-HrwYpFbDwWjdmhJQiZiTRa_zXPPOgUTdmbsaq4meo.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/L-HrwYpFbDwWjdmhJQiZiTRa_zXPPOgUTdmbsaq4meo.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/L-HrwYpFbDwWjdmhJQiZiTRa_zXPPOgUTdmbsaq4meo.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn TitanLeagueII() -> Self {
        Self {
            id: LeagueKind::TitanLeagueII,
            name: "Titan League II".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/llpWocHlOoFliwyaEx5Z6dmoZG4u4NmxwpF-Jg7su7Q.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/llpWocHlOoFliwyaEx5Z6dmoZG4u4NmxwpF-Jg7su7Q.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/llpWocHlOoFliwyaEx5Z6dmoZG4u4NmxwpF-Jg7su7Q.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn TitanLeagueI() -> Self {
        Self {
            id: LeagueKind::TitanLeagueI,
            name: "Titan League I".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png".to_string()),
            },
        }
    }

    #[must_use]
    pub fn LegendLeague() -> Self {
        Self {
            id: LeagueKind::LegendLeague,
            name: "Legend League".to_string(),
            icon_urls: LeagueIconUrls {
                small: "https://api-assets.clashofclans.com/leagues/72/R2zmhyqQ0_lKcDR5EyghXCxgyC9mm_mVMIjAbmGoZtw.png".to_string(), 
                tiny: "https://api-assets.clashofclans.com/leagues/36/R2zmhyqQ0_lKcDR5EyghXCxgyC9mm_mVMIjAbmGoZtw.png".to_string(),
                medium: Some("https://api-assets.clashofclans.com/leagues/288/R2zmhyqQ0_lKcDR5EyghXCxgyC9mm_mVMIjAbmGoZtw.png".to_string()),
            },
        }
    }
}
