use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BadgeUrls {
    #[serde(rename = "small")]
    pub small: String,

    #[serde(rename = "large")]
    pub large: String,

    #[serde(rename = "medium")]
    pub medium: String,
}
