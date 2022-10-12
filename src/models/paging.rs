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
    fn new() -> Self {
        Self { paging: Paging { cursor: Cursor { before: None, after: None } } }
    }
    #[must_use] pub fn before(mut self, before: i32) -> Self {
        self.paging.cursor.set_before(before);
        self
    }
    #[must_use] pub fn after(mut self, after: i32) -> Self {
        self.paging.cursor.set_after(after);
        self
    }
    #[must_use] pub fn build(self) -> Paging {
        self.paging
    }
}

impl Paging {
    #[must_use] pub fn new() -> Self {
        Self { cursor: Cursor::new() }
    }

    #[must_use] pub fn is_some(&self) -> bool {
        self.cursor.is_some()
    }

    #[must_use] pub fn is_none(&self) -> bool {
        self.cursor.is_none()
    }

    #[must_use] pub fn to_vec(&self) -> Vec<(&str, String)> {
        self.cursor.to_vec()
    }

    #[must_use] pub fn builder() -> PagingBuilder {
        PagingBuilder::new()
    }
}

impl Default for Paging {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    #[must_use] pub fn new() -> Self {
        Self { before: None, after: None }
    }
    fn set_before(&mut self, before: i32) {
        // {"pos": before} base64 encoded and cant be less than 0
        self.before = Some(base64::encode(&format!("{{\"pos\":{before}}}")));
    }
    fn set_after(&mut self, after: i32) {
        // {"pos": after} base64 encoded and cant be less than 0
        self.after = Some(base64::encode(&format!("{{\"pos\":{after}}}")));
    }

    #[must_use] pub fn is_some(&self) -> bool {
        self.before.is_some() || self.after.is_some()
    }

    #[must_use] pub fn is_none(&self) -> bool {
        self.before.is_none() && self.after.is_none()
    }

    #[must_use] pub fn to_vec(&self) -> Vec<(&str, String)> {
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
