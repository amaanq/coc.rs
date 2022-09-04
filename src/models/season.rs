use serde::{Deserialize, Serialize};
use time::Month;

#[derive(Debug)]
pub enum SeasonError {
    InvalidSeason,
    ParseFailed(String),
}

#[derive(Debug)]
pub struct SeasonBuilder {
    season: Season,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    #[serde(rename = "id")]
    id: String,
    #[serde(skip_serializing)]
    year: i32,
    #[serde(skip_deserializing, skip_serializing, default = "Season::default_month")]
    month: Month,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreviousSeasonData {
    pub id: String,
    pub rank: i32,
    pub trophies: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestSeasonData {
    pub id: String,
    pub rank: i32,
    pub trophies: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreviousVersusSeasonData {
    pub id: String,
    pub rank: i32,
    pub trophies: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestVersusSeasonData {
    pub id: String,
    pub rank: i32,
    pub trophies: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurrentSeasonData {
    pub id: Option<String>,
    pub rank: Option<i32>,
    pub trophies: i32,
}

impl From<std::num::ParseIntError> for SeasonError {
    fn from(err: std::num::ParseIntError) -> Self {
        SeasonError::ParseFailed(err.to_string())
    }
}

impl SeasonBuilder {
    fn new() -> SeasonBuilder {
        SeasonBuilder { season: Season { id: String::new(), year: 2015, month: Month::July } }
    }

    pub fn year(mut self, year: i32) -> SeasonBuilder {
        self.season.year = year;
        self
    }

    pub fn month(mut self, month: Month) -> SeasonBuilder {
        self.season.month = month;
        self
    }

    pub fn build(self) -> Season {
        self.season
    }
}

impl Season {
    pub fn from_string(season: String) -> Result<Season, SeasonError> {
        let mut season_split = season.split('-');
        Ok(Season {
            id: season.clone(),
            year: season_split.next().unwrap().parse::<i32>()?,
            month: Month::try_from(season_split.next().unwrap().parse::<i32>()? as u8).unwrap(),
        })
    }

    pub fn default_month() -> Month {
        Month::July
    }

    pub fn builder() -> SeasonBuilder {
        SeasonBuilder::new()
    }
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{:02}", self.year, self.month as i32)
    }
}
