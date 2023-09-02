use serde::{Deserialize, Serialize};
use time::Month;

#[derive(Debug, Clone)]
pub enum SeasonError {
    InvalidSeason,
    ParseFailed(String),
}

#[derive(Debug, Clone)]
pub struct SeasonBuilder {
    season: Season,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    #[serde(rename = "id")]
    id: String,
    #[serde(skip_deserializing, skip_serializing)]
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
        Self::ParseFailed(err.to_string())
    }
}

impl SeasonBuilder {
    const fn new() -> Self {
        Self { season: Season { id: String::new(), year: 2015, month: Month::July } }
    }

    #[must_use]
    pub const fn year(mut self, year: i32) -> Self {
        self.season.year = year;
        self
    }

    #[must_use]
    pub const fn month(mut self, month: Month) -> Self {
        self.season.month = month;
        self
    }

    #[must_use]
    pub fn build(mut self) -> Season {
        self.season.id = format!("{}-{:02}", self.season.year, self.season.month as i32);
        self.season
    }
}

impl Season {
    #[must_use]
    pub const fn default_month() -> Month {
        Month::July
    }

    #[must_use]
    pub const fn builder() -> SeasonBuilder {
        SeasonBuilder::new()
    }
}

impl std::str::FromStr for Season {
    type Err = SeasonError;

    fn from_str(season: &str) -> Result<Self, Self::Err> {
        let mut season_split = season.split('-');
        Ok(Self {
            id: season.to_string(),
            year: season_split.next().ok_or(SeasonError::InvalidSeason)?.parse()?,
            month: Month::try_from(
                season_split.next().ok_or(SeasonError::InvalidSeason)?.parse::<u8>()?,
            )
            .unwrap(),
        })
    }
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut season_split = self.id.split('-');
        let year = season_split.next().unwrap().parse::<i32>().unwrap();
        let month = Month::try_from(season_split.next().unwrap().parse::<u8>().unwrap()).unwrap();
        write!(f, "{year}-{:02}", month as i32)
    }
}
