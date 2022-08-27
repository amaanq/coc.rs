use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BadgeUrls {
    #[serde(rename = "small")]
    small: String,

    #[serde(rename = "large")]
    large: String,

    #[serde(rename = "medium")]
    medium: String,
}

impl BadgeUrls {
    pub fn small(&self) -> &str {
        &self.small
    }
    pub fn large(&self) -> &str {
        &self.large
    }
    pub fn medium(&self) -> &str {
        &self.medium
    }
}
