use super::passport_record::PassportRecord;
use super::passport_record_attribute_key::PassportRecordAttributeKey;
use anyhow::{Context, Error, Result};
use regex::Regex;

lazy_static! {
    /// Matches passport record hair colors.
    static ref PASSPORT_RECORD_HAIR_COLOR_PATTERN: Regex =
        Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    /// Matches passport record heights.
    ///
    /// | capture group    | index |
    /// |------------------|-------|
    /// | measurement      | 1     |
    /// | unit             | 2     |
    static ref PASSPORT_RECORD_HEIGHT_PATTERN: Regex =
        Regex::new(r"^(\d*\.?\d+)(cm|in)$").unwrap();

    /// Matches passport record passport ids.
    static ref PASSPORT_RECORD_PASSPORT_ID_PATTERN: Regex =
        Regex::new(r"^\d{9}$").unwrap();
}

/// Enumerates every password record attribute label.
#[derive(Debug)]
pub struct ValidatedPassportRecord {
    /// `"byr"` passport record attribute.
    birth_year: u32,
    /// `"cid"` passport record attribute.
    country_id: Option<String>,
    /// `"eyr"` passport record attribute.
    expiration_year: u32,
    /// `"ecl"` passport record attribute.
    eye_color: ValidatedPassportRecordEyeColor,
    /// `"hcl"` passport record attribute.
    hair_color: String,
    /// `"hgt"` passport record attribute.
    height: ValidatedPassportRecordHeight,
    /// `"iyr"` passport record attribute.
    issue_year: u32,
    /// `"pid"` passport record attribute.
    passport_id: String,
}

impl ValidatedPassportRecord {
    /// Inspects the referenced `passport_record`, returning its validated form.
    pub fn from_passport_record(
        passport_record: &PassportRecord,
    ) -> Result<ValidatedPassportRecord> {
        let birth_year_string = passport_record
            .value_of(PassportRecordAttributeKey::BirthYear)
            .context("Birth year is missing")?;
        let birth_year = birth_year_string
            .parse::<u32>()
            .with_context(|| format!("\"{}\" is not a valid number", birth_year_string))?;
        if !(1920..2003).contains(&birth_year) {
            return Err(Error::msg(format!(
                "{} is not a valid birth year",
                birth_year
            )));
        }

        let country_id = passport_record
            .value_of(PassportRecordAttributeKey::CountryId)
            .map(|country_id| country_id.to_owned());

        let expiration_year_string = passport_record
            .value_of(PassportRecordAttributeKey::ExpirationYear)
            .context("Expiration year is missing")?;
        let expiration_year = expiration_year_string
            .parse::<u32>()
            .with_context(|| format!("\"{}\" is not a valid number", expiration_year_string))?;
        if !(2020..2031).contains(&expiration_year) {
            return Err(Error::msg(format!(
                "{} is not a valid expiration year",
                expiration_year,
            )));
        }

        let eye_color_string = passport_record
            .value_of(PassportRecordAttributeKey::EyeColor)
            .context("Eye color is missing")?;
        let eye_color = ValidatedPassportRecordEyeColor::from_string(eye_color_string)?;

        let hair_color = passport_record
            .value_of(PassportRecordAttributeKey::HairColor)
            .context("Hair color is missing")?;
        if !PASSPORT_RECORD_HAIR_COLOR_PATTERN.is_match(hair_color) {
            return Err(Error::msg(format!(
                "\"{}\" is not a valid hair color",
                hair_color,
            )));
        }

        let height_string = passport_record
            .value_of(PassportRecordAttributeKey::Height)
            .context("Height is missing")?;
        let height = ValidatedPassportRecordHeight::from_string(height_string)?;

        let issue_year_string = passport_record
            .value_of(PassportRecordAttributeKey::IssueYear)
            .context("Issue year is missing")?;
        let issue_year = issue_year_string
            .parse::<u32>()
            .with_context(|| format!("\"{}\" is not a valid number", issue_year_string))?;
        if !(2010..2021).contains(&issue_year) {
            return Err(Error::msg(format!(
                "{} is not a valid issue year",
                issue_year,
            )));
        }

        let passport_id = passport_record
            .value_of(PassportRecordAttributeKey::PassportId)
            .context("Passport id is missing")?;
        if !PASSPORT_RECORD_PASSPORT_ID_PATTERN.is_match(passport_id) {
            return Err(Error::msg(format!(
                "\"{}\" is not a valid passport id",
                passport_id,
            )));
        }

        Ok(ValidatedPassportRecord {
            birth_year,
            country_id,
            expiration_year,
            eye_color,
            hair_color: hair_color.to_owned(),
            height,
            issue_year,
            passport_id: passport_id.to_owned(),
        })
    }
}

/// Enumerates all valid human heights that can appear in a passport record.
#[derive(Debug)]
pub enum ValidatedPassportRecordHeight {
    Centimeters(f64),
    Inches(f64),
}

impl ValidatedPassportRecordHeight {
    /// Returns the `ValidatedPassportRecordHeight` equivalent to the given `string`.
    fn from_string(string: &str) -> Result<ValidatedPassportRecordHeight> {
        let capture_groups = PASSPORT_RECORD_HEIGHT_PATTERN
            .captures(string)
            .with_context(|| format!("\"{}\" is not a valid passport record height", string))?;
        let measurement_text = &capture_groups[1];
        let measurement_unit_text = &capture_groups[2];

        let measurement = measurement_text.parse::<f64>().with_context(|| {
            format!(
                "\"{}\" is not a valid passport record height measurement",
                measurement_text
            )
        })?;

        match measurement_unit_text {
            "cm" => {
                if !(150.0..194.0).contains(&measurement) {
                    return Err(Error::msg(format!(
                        "{} is not a valid measurement in centimeters",
                        measurement,
                    )));
                }

                Ok(ValidatedPassportRecordHeight::Centimeters(measurement))
            }
            "in" => {
                if !(59.0..77.0).contains(&measurement) {
                    return Err(Error::msg(format!(
                        "{} is not a valid measurement in inches",
                        measurement,
                    )));
                }

                Ok(ValidatedPassportRecordHeight::Inches(measurement))
            }
            _ => Err(Error::msg(format!(
                "\"{}\" is not a valid passport record height measurement unit",
                measurement_unit_text
            ))),
        }
    }
}

/// Enumerates all valid human eye colors that can appear in a passport record.
#[derive(Debug)]
pub enum ValidatedPassportRecordEyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl ValidatedPassportRecordEyeColor {
    /// Returns the `ValidatedPassportRecordEyeColor` equivalent to the given `string`.
    fn from_string(string: &str) -> Result<ValidatedPassportRecordEyeColor> {
        match string {
            "amb" => Ok(ValidatedPassportRecordEyeColor::Amber),
            "blu" => Ok(ValidatedPassportRecordEyeColor::Blue),
            "brn" => Ok(ValidatedPassportRecordEyeColor::Brown),
            "gry" => Ok(ValidatedPassportRecordEyeColor::Gray),
            "grn" => Ok(ValidatedPassportRecordEyeColor::Green),
            "hzl" => Ok(ValidatedPassportRecordEyeColor::Hazel),
            "oth" => Ok(ValidatedPassportRecordEyeColor::Other),
            _ => Err(Error::msg(format!(
                "\"{}\" is not a valid passport record eye color",
                string
            ))),
        }
    }
}
