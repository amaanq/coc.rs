use chrono::TimeZone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoldPass {
    start_time: String,
    end_time: String,
}

impl GoldPass {
    #[must_use]
    pub fn start_time(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_utc_datetime(
            &chrono::NaiveDateTime::parse_from_str(&self.start_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
        )
    }

    #[must_use]
    pub fn end_time(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_utc_datetime(
            &chrono::NaiveDateTime::parse_from_str(&self.end_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
        )
    }
}
