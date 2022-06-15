#[derive(Debug, Serialize, Deserialize)]
struct Paging {
    #[serde(rename = "cursors")]
    cursors: Cursors,
}

#[derive(Debug, Serialize, Deserialize)]
enum Cursors {
    #[serde(rename = "before")]
    Before(String),
    #[serde(rename = "after")]
    After(String),
}

impl Paging {
    pub fn before(&self) -> &str {
        match self.cursors {
            Cursors::Before(ref s) => s,
            _ => panic!("Paging::before: not a before cursor"),
        }
    }
}