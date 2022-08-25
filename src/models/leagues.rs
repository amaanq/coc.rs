use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum League {
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
pub enum WarLeague {
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

#[derive(Debug)]
pub enum SeasonError {
    InvalidSeason,
}

#[derive(Debug)]
pub struct SeasonBuilder {
    season: Season,
}

impl SeasonBuilder {
    const YEAR_MIN: i32 = 2015;
    const YEAR_MAX: i32 = 2022;
    const MONTH_MIN: i32 = 1;
    const MONTH_MAX: i32 = 12;
    const CURRENT_YEAR_MONTH_MAX: i32 = 8;

    pub fn new() -> SeasonBuilder {
        SeasonBuilder {
            season: Season::default(),
        }
    }

    pub fn year(mut self, year: i32) -> SeasonBuilder {
        self.season.year = year;
        self
    }

    pub fn month(mut self, month: i32) -> SeasonBuilder {
        self.season.month = month;
        self
    }

    pub fn build(&mut self) -> Result<String, SeasonError> {
        // first check if year is between Self::YEAR_MIN and Self::YEAR_MAX
        // if year IS Self::YEAR_MAX, check if month is less than or equal to Self::CURRENT_YEAR_MONTH_MAX, else just check if between Self::MONTH_MIN and Self::MONTH_MAX
        // then call .to_string() on the season struct
        if self.season.year < Self::YEAR_MIN || self.season.year > Self::YEAR_MAX {
            return Err(SeasonError::InvalidSeason);
        }
        if self.season.year == Self::YEAR_MAX {
            if self.season.month < Self::MONTH_MIN
                || self.season.month > Self::CURRENT_YEAR_MONTH_MAX
            {
                return Err(SeasonError::InvalidSeason);
            } else {
                return Ok(self.season.to_string());
            }
        } else {
            if self.season.month < Self::MONTH_MIN || self.season.month > Self::MONTH_MAX {
                return Err(SeasonError::InvalidSeason);
            } else {
                return Ok(self.season.to_string());
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Season {
    #[serde(rename = "id")]
    id: String,
    #[serde(skip_serializing)]
    year: i32,
    #[serde(skip_serializing)]
    month: i32,
}

impl Season {
    pub fn to_string(&mut self) -> String {
        //YYYY-MM
        self.id = format!("{}-{}", self.year, self.month);
        self.id.clone()
    }
}
