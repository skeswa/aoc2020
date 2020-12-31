use anyhow::{Context, Error, Result};
use regex::Regex;

lazy_static! {
    /// Matches password database entries.
    ///
    /// | capture group    | index |
    /// |------------------|-------|
    /// | min. repetitions | 1     |
    /// | max. repetitions | 2     |
    /// | repeated letter  | 3     |
    /// | password         | 4     |
    static ref PASSWORD_DATABASE_ENTRY_PATTERN: Regex =
        Regex::new(r"^(\d+)-(\d+)\s+([a-z]):\s+([a-z]+)$").unwrap();
}
/// Stores password information for the North Pole Toboggan Rental Shop
/// computer system.
#[derive(Debug)]
pub struct PasswordDatabase {
    /// Password wrapped by this `PasswordDatabaseEntry`.
    pub entries: Vec<PasswordDatabaseEntry>,
}

impl PasswordDatabase {
    /// Constructs a new `PasswordDatabase` based on the provided
    /// `password_database_file_contents`.
    ///
    /// For more on the format of `password_database_file_contents`,
    /// see https://adventofcode.com/2020/day/2.
    pub fn from_file(password_database_file_contents: &str) -> Result<PasswordDatabase> {
        let entries = password_database_file_contents
            .lines()
            .map(|line| PasswordDatabaseEntry::from_text(line))
            .collect::<Result<Vec<PasswordDatabaseEntry>>>()?;

        Ok(PasswordDatabase { entries })
    }
}

/// A single entry of the corporate password database.
#[derive(Debug)]
pub struct PasswordDatabaseEntry {
    /// Password wrapped by this `PasswordDatabaseEntry`.
    password: String,
    /// Rule validating `password`.
    rule: PasswordFormatRule,
}

impl PasswordDatabaseEntry {
    /// Returns `true` if this entry's `rule` applies to its `password`.
    pub fn is_valid(&self) -> bool {
        let repetitions = self
            .password
            .chars()
            .filter(|password_char| *password_char == self.rule.letter)
            .count() as i64;

        repetitions >= self.rule.min_repetitions && repetitions <= self.rule.max_repetitions
    }
}

/// Describes a valid password.
#[derive(Debug)]
struct PasswordFormatRule {
    /// The letter to which `maxRepetitions` and `minRepetitions` refer.
    letter: char,
    /// Maximum number of times that `letter` can appear in a password.
    max_repetitions: i64,
    /// Minimum number of times that `letter` can appear in a password.
    min_repetitions: i64,
}

impl PasswordDatabaseEntry {
    /// Constructs a new `PasswordDatabaseEntry` from the provided `text`.
    ///
    /// For more on the format of `text`,
    /// see https://adventofcode.com/2020/day/2.
    fn from_text(text: &str) -> Result<PasswordDatabaseEntry, Error> {
        let capture_groups = PASSWORD_DATABASE_ENTRY_PATTERN
            .captures(text)
            .with_context(|| format!("Failed to parse password database entry text: {}", text))?;

        let min_repetitions = capture_groups.get(1).unwrap().as_str().parse::<i64>()?;
        let max_repetitions = capture_groups.get(2).unwrap().as_str().parse::<i64>()?;
        let letter = capture_groups
            .get(3)
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let password = capture_groups.get(4).unwrap().as_str();

        Ok(PasswordDatabaseEntry {
            password: password.to_owned(),
            rule: PasswordFormatRule {
                letter,
                max_repetitions,
                min_repetitions,
            },
        })
    }
}
