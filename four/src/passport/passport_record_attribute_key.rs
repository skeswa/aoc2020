use anyhow::{Error, Result};

/// Enumerates every password record attribute label.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum PassportRecordAttributeKey {
    BirthYear,
    CountryId,
    ExpirationYear,
    EyeColor,
    HairColor,
    Height,
    IssueYear,
    PassportId,
}

impl PassportRecordAttributeKey {
    /// Returns the `PassportRecordAttributeKey` represented by the given `string`.
    pub fn from_string(string: &str) -> Result<PassportRecordAttributeKey> {
        match string {
            "byr" => Ok(PassportRecordAttributeKey::BirthYear),
            "cid" => Ok(PassportRecordAttributeKey::CountryId),
            "eyr" => Ok(PassportRecordAttributeKey::ExpirationYear),
            "ecl" => Ok(PassportRecordAttributeKey::EyeColor),
            "hcl" => Ok(PassportRecordAttributeKey::HairColor),
            "hgt" => Ok(PassportRecordAttributeKey::Height),
            "iyr" => Ok(PassportRecordAttributeKey::IssueYear),
            "pid" => Ok(PassportRecordAttributeKey::PassportId),
            _ => Err(Error::msg(format!(
                "\"{}\" is not a valid attribute key",
                string
            ))),
        }
    }
}
