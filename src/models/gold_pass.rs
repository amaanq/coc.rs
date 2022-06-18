use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct GoldPass {
    #[serde(rename = "stratTime")]
    start_time: String, 
    #[serde(rename = "endTime")]
    end_time: String,
}

impl GoldPass {
    pub fn start_time(&self) -> &str{ &self.start_time }
    pub fn end_time(&self) -> &str { &self.end_time }
}
