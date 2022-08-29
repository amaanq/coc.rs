use serde::{Deserialize, Serialize};

use super::*;
use crate::icon_urls::LeagueIconUrls;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LeagueKind {
    Unranked = 29000000,
    BronzeLeagueIII = 29000001,
    BronzeLeagueII = 29000002,
    BronzeLeagueI = 29000003,
    SilverLeagueIII = 29000004,
    SilverLeagueII = 29000005,
    SilverLeagueI = 29000006,
    GoldLeagueIII = 29000007,
    GoldLeagueII = 29000008,
    GoldLeagueI = 29000009,
    CrystalLeagueIII = 29000010,
    CrystalLeagueII = 29000011,
    CrystalLeagueI = 29000012,
    MasterLeagueIII = 29000013,
    MasterLeagueII = 29000014,
    MasterLeagueI = 29000015,
    ChampionLeagueIII = 29000016,
    ChampionLeagueII = 29000017,
    ChampionLeagueI = 29000018,
    TitanLeagueIII = 29000019,
    TitanLeagueII = 29000020,
    TitanLeagueI = 29000021,
    LegendLeague = 29000022,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum WarLeagueKind {
    Unranked = 48000000,
    BronzeLeagueIII = 48000001,
    BronzeLeagueII = 48000002,
    BronzeLeagueI = 48000003,
    SilverLeagueIII = 48000004,
    SilverLeagueII = 48000005,
    SilverLeagueI = 48000006,
    GoldLeagueIII = 48000007,
    GoldLeagueII = 48000008,
    GoldLeagueI = 48000009,
    CrystalLeagueIII = 48000010,
    CrystalLeagueII = 48000011,
    CrystalLeagueI = 48000012,
    MasterLeagueIII = 48000013,
    MasterLeagueII = 48000014,
    MasterLeagueI = 48000015,
    ChampionLeagueIII = 48000016,
    ChampionLeagueII = 48000017,
    ChampionLeagueI = 48000018,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: LeagueKind,
    pub name: String,
    pub icon_urls: icon_urls::LeagueIconUrls,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarLeague {
    pub id: WarLeagueKind,
    pub name: String,
}

impl League {
    pub fn Unranked() -> Self {
        League {
            id: LeagueKind::Unranked,
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
            id: LeagueKind::BronzeLeagueIII,
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
            id: LeagueKind::BronzeLeagueII,
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
            id: LeagueKind::BronzeLeagueI,
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
            id: LeagueKind::SilverLeagueIII,
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
            id: LeagueKind::SilverLeagueII,
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
            id: LeagueKind::SilverLeagueI,
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
            id: LeagueKind::GoldLeagueIII,
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
            id: LeagueKind::GoldLeagueII,
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
            id: LeagueKind::GoldLeagueI,
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
            id: LeagueKind::CrystalLeagueIII,
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
            id: LeagueKind::CrystalLeagueII,
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
            id: LeagueKind::CrystalLeagueI,
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
            id: LeagueKind::MasterLeagueIII,
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
            id: LeagueKind::MasterLeagueII,
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
            id: LeagueKind::MasterLeagueI,
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
            id: LeagueKind::ChampionLeagueIII,
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
            id: LeagueKind::ChampionLeagueII,
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
            id: LeagueKind::ChampionLeagueI,
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
            id: LeagueKind::TitanLeagueIII,
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
            id: LeagueKind::TitanLeagueII,
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
            id: LeagueKind::TitanLeagueI,
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
