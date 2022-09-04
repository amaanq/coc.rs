use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Paging {
    #[serde(rename = "cursors")]
    cursor: Cursor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cursor {
    before: Option<String>,
    after: Option<String>,
}

pub struct PagingBuilder {
    paging: Paging,
}

impl PagingBuilder {
    fn new() -> PagingBuilder {
        PagingBuilder { paging: Paging { cursor: Cursor { before: None, after: None } } }
    }
    pub fn before(mut self, before: i32) -> PagingBuilder {
        self.paging.cursor.set_before(before);
        self
    }
    pub fn after(mut self, after: i32) -> PagingBuilder {
        self.paging.cursor.set_after(after);
        self
    }
    pub fn build(self) -> Paging {
        self.paging
    }
}

impl Paging {
    pub fn new() -> Self {
        Self { cursor: Cursor::new() }
    }

    pub fn is_some(&self) -> bool {
        self.cursor.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.cursor.is_none()
    }

    pub fn to_vec(&self) -> Vec<(&str, String)> {
        self.cursor.to_vec()
    }

    pub fn builder() -> PagingBuilder {
        PagingBuilder::new()
    }
}

impl Default for Paging {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self { before: None, after: None }
    }
    fn set_before(&mut self, before: i32) {
        // {"pos": before} base64 encoded and cant be less than 0
        self.before = Some(base64::encode(&format!("{{\"pos\":{}}}", before)));
    }
    fn set_after(&mut self, after: i32) {
        // {"pos": after} base64 encoded and cant be less than 0
        self.after = Some(base64::encode(&format!("{{\"pos\":{}}}", after)));
    }

    pub fn is_some(&self) -> bool {
        self.before.is_some() || self.after.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.before.is_none() && self.after.is_none()
    }

    pub fn to_vec(&self) -> Vec<(&str, String)> {
        let mut vec = Vec::new();
        if let Some(ref before) = self.before {
            vec.push(("before", before.clone()));
        }
        if let Some(ref after) = self.after {
            vec.push(("after", after.clone()));
        }
        vec
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}
