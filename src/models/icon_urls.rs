use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelIconUrls {
    pub small: String,
    pub medium: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueIconUrls {
    pub small: String,
    pub tiny: String,
    pub medium: Option<String>,
}
