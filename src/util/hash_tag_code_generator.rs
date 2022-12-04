use lazy_static::lazy_static;

use crate::error::APIError;

use super::{LogicLong, LogicLongToCodeConverterUtil};

pub struct HashTagCodeGenerator {
    code_converter_util: LogicLongToCodeConverterUtil,
}

impl HashTagCodeGenerator {
    pub const CONVERSION_CHARS: &str = "0289PYLQGRJCUV";
    pub const CONVERSION_TAG: &str = "#";

    #[must_use]
    pub fn new() -> Self {
        Self {
            code_converter_util: LogicLongToCodeConverterUtil::new(
                Self::CONVERSION_TAG,
                Self::CONVERSION_CHARS,
            ),
        }
    }

    #[must_use]
    pub fn to_code(&self, logic_long: LogicLong) -> String {
        self.code_converter_util.to_code(logic_long)
    }

    pub fn to_id(&self, value: &str) -> Result<LogicLong, APIError> {
        let id = self.code_converter_util.to_id(value);

        if Self::is_id_valid(id) {
            Ok(id)
        } else {
            Err(APIError::InvalidTag(value.to_string()))
        }
    }

    #[must_use]
    pub const fn is_id_valid(id: LogicLong) -> bool {
        id.get_higher_int() != -1 && id.get_lower_int() != -1
    }
}

impl Default for HashTagCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    pub static ref HASH_TAG_CODE_GENERATOR: HashTagCodeGenerator = HashTagCodeGenerator::new();
}
