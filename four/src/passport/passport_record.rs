use super::passport_record_attribute_key::PassportRecordAttributeKey;
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    /// Matches passport record attributes.
    ///
    /// | capture group    | index |
    /// |------------------|-------|
    /// | key              | 1     |
    /// | value            | 2     |
    static ref PASSPORT_RECORD_ATTRIBUTE_PATTERN: Regex =
        Regex::new(r"([a-z]+):(#?[a-zA-Z0-9]+)").unwrap();
}

/// Represents information about a single passport.
#[derive(Debug)]
pub struct PassportRecord {
    attributes: HashMap<PassportRecordAttributeKey, String>,
}

impl PassportRecord {
    pub fn from_text(text: &str) -> Result<PassportRecord> {
        let mut attributes: HashMap<PassportRecordAttributeKey, String> = HashMap::new();

        for capture_groups in PASSPORT_RECORD_ATTRIBUTE_PATTERN.captures_iter(text) {
            let key = PassportRecordAttributeKey::from_text(&capture_groups[1])?;
            let value = capture_groups[2].to_string();

            attributes.insert(key, value);
        }

        Ok(PassportRecord { attributes })
    }

    pub fn is_valid(&self) -> bool {
        self.attributes.len() == 8
            || (self.attributes.len() == 7
                && !self
                    .attributes
                    .contains_key(&PassportRecordAttributeKey::CountryId))
    }
}
