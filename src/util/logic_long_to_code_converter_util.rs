use super::LogicLong;

#[derive(Debug)]
pub struct LogicLongToCodeConverterUtil {
    hash_tag: String,
    conversion_chars: String,
}

impl LogicLongToCodeConverterUtil {
    pub fn new(hash_tag: &str, conversion_chars: &str) -> Self {
        Self { hash_tag: hash_tag.to_string(), conversion_chars: conversion_chars.to_string() }
    }

    pub fn to_code(&self, logic_long: LogicLong) -> String {
        let high_value = i64::from(logic_long.get_higher_int());

        if high_value < 256 {
            return self.hash_tag.clone()
                + &self.convert((i64::from(logic_long.get_lower_int()) << 8) | high_value);
        }

        #[cfg(feature = "tracing")]
        tracing::trace!("Cannot convert the code to string. Higher int value too large");
        String::new()
    }

    pub fn to_id(&self, code: &str) -> LogicLong {
        if code.len() < 14 {
            let id_code = code.replace(self.hash_tag.as_str(), "");
            let id = self.convert_code(&id_code);

            if id != -1 {
                return LogicLong::new((id % 256) as i32, ((id >> 8) & 0x7FFF_FFFF) as i32);
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::trace!("Cannot convert the string to code. String is too long.");
        }

        LogicLong::new(-1, -1)
    }

    pub fn convert_code(&self, code: &str) -> i64 {
        let mut id: i64 = 0;
        let conversion_chars_count = self.conversion_chars.len();
        let code_chars_count = code.len();

        for i in 0..code_chars_count {
            let char_index = self.conversion_chars.find(code.chars().nth(i).unwrap());

            if char_index.is_none() {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Cannot convert the string to code. String contains invalid character(s)."
                );
                id = -1;
                break;
            }

            id = id * conversion_chars_count as i64 + char_index.unwrap() as i64;
        }

        id
    }

    fn convert(&self, mut value: i64) -> String {
        if value > -1 {
            let conversion_chars_count = self.conversion_chars.len();
            let mut code = [' '; 12];

            for i in (0..12).rev() {
                let index = (value % conversion_chars_count as i64) as usize;
                if let Some(char) = self.conversion_chars.get(index..=index) {
                    code[i] = char.chars().next().unwrap();
                }
                value /= conversion_chars_count as i64;

                if value == 0 || i == 0 {
                    let start_idx = code.iter().take_while(|&&c| c == ' ').count();
                    let mut result = String::with_capacity(12 - start_idx);
                    for ch in code[start_idx..].iter() {
                        result.push(*ch);
                    }
                    return result;
                }
            }
        }

        #[cfg(feature = "tracing")]
        tracing::trace!("LogicLongToCodeConverter: value to convert cannot be negative");

        String::new()
    }
}
