use std::str::FromStr;

use crate::error::APIError;

use super::hash_tag_code_generator::HASH_TAG_CODE_GENERATOR;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogicLong {
    pub high_integer: i32,
    pub low_integer: i32,
}

impl LogicLong {
    pub fn new(high_integer: i32, low_integer: i32) -> Self {
        Self { high_integer, low_integer }
    }

    pub fn get_higher_int(&self) -> i32 {
        self.high_integer
    }

    pub fn get_lower_int(&self) -> i32 {
        self.low_integer
    }
}

impl FromStr for LogicLong {
    type Err = APIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HASH_TAG_CODE_GENERATOR.to_id(&s.to_uppercase().replace('O', "0"))
    }
}

impl ToString for LogicLong {
    fn to_string(&self) -> String {
        HASH_TAG_CODE_GENERATOR.to_code(*self)
    }
}
