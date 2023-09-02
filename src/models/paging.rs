use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use serde::{Deserialize, Serialize};

pub(crate) const BASE64_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

#[derive(Debug, Serialize, Deserialize)]
pub struct Paging {
    #[serde(rename = "cursors")]
    cursor: Cursor,
}

impl std::fmt::Display for Paging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Paging {{ ")?;
        writeln!(f, "cursor: {}", self.cursor)?;
        writeln!(f, "}}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cursor {
    before: Option<String>,
    after: Option<String>,
}

impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cursor {{ ")?;
        if let Some(before) = &self.before {
            writeln!(f, "before: {before}")?;
        }
        if let Some(after) = &self.after {
            writeln!(f, "after: {after}")?;
        }
        writeln!(f, "}}")
    }
}

pub struct PagingBuilder {
    paging: Paging,
}

impl PagingBuilder {
    const fn new() -> Self {
        Self { paging: Paging { cursor: Cursor { before: None, after: None } } }
    }

    #[must_use]
    pub fn before(mut self, before: i32) -> Self {
        self.paging.cursor.set_before(before);
        self
    }

    #[must_use]
    pub fn after(mut self, after: i32) -> Self {
        self.paging.cursor.set_after(after);
        self
    }

    #[must_use]
    pub fn build(self) -> Paging {
        self.paging
    }
}

impl Paging {
    #[must_use]
    pub const fn new() -> Self {
        Self { cursor: Cursor::new() }
    }

    #[must_use]
    pub const fn is_some(&self) -> bool {
        self.cursor.is_some()
    }

    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.cursor.is_none()
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<(&str, String)> {
        self.cursor.to_vec()
    }

    #[must_use]
    pub const fn builder() -> PagingBuilder {
        PagingBuilder::new()
    }
}

impl Default for Paging {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    #[must_use]
    pub const fn new() -> Self {
        Self { before: None, after: None }
    }

    fn set_before(&mut self, before: i32) {
        self.before = Some(BASE64_ENGINE.encode(format!("{{\"pos\":{before}}}")));
    }

    fn set_after(&mut self, after: i32) {
        self.after = Some(BASE64_ENGINE.encode(format!("{{\"pos\":{after}}}")));
    }

    #[must_use]
    pub const fn is_some(&self) -> bool {
        self.before.is_some() || self.after.is_some()
    }

    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.before.is_none() && self.after.is_none()
    }

    #[must_use]
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
