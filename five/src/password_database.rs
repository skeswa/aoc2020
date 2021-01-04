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
    /// Data used to validate `password`.
    validation_metadata: PasswordValidationMetadata,
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

        let first_parameter = capture_groups.get(1).unwrap().as_str().parse::<i64>()?;
        let second_parameter = capture_groups.get(2).unwrap().as_str().parse::<i64>()?;
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
            validation_metadata: PasswordValidationMetadata {
                letter,
                parameters: (first_parameter, second_parameter),
            },
        })
    }

    /// Returns `true` if this entry's `rule` applies to its `password`.
    pub fn is_valid(&self, strategy: PasswordValidationStrategy) -> bool {
        match strategy {
            PasswordValidationStrategy::LetterRepetitionRange => {
                let PasswordDatabaseEntry {
                    password,
                    validation_metadata:
                        PasswordValidationMetadata {
                            letter,
                            parameters: (min_repetitions, max_repetitions),
                        },
                } = self;

                let repetitions = password
                    .chars()
                    .filter(|password_char| *password_char == *letter)
                    .count() as i64;

                repetitions >= *min_repetitions && repetitions <= *max_repetitions
            }
            PasswordValidationStrategy::LetterPositions => {
                let PasswordDatabaseEntry {
                    password,
                    validation_metadata:
                        PasswordValidationMetadata {
                            letter,
                            parameters: (first_position, second_position),
                        },
                } = self;

                let does_first_position_match = does_letter_match(letter, password, first_position);
                let does_second_position_match =
                    does_letter_match(letter, password, second_position);

                (does_first_position_match || does_second_position_match)
                    && !(does_first_position_match && does_second_position_match)
            }
        }
    }
}

/// Enumerates every way that `PasswordValidationMetadata` can be used.
pub enum PasswordValidationStrategy {
    /// Interprets `PasswordValidationMetadata#parameters` as a repetition
    /// range where the first value is the (inclusive) minimum number of
    /// letter repetitions in a valid password and the second value is the
    /// (inclusive) maximum number of repetitions in a valid password.
    LetterRepetitionRange,
    /// Interprets `PasswordValidationMetadata#parameters` as legitimate
    /// positions for the letter in a valid password.
    LetterPositions,
}

/// Describes a valid password.
#[derive(Debug)]
struct PasswordValidationMetadata {
    /// The letter to which `maxRepetitions` and `minRepetitions` refer.
    letter: char,
    /// Values configuring validation.
    parameters: (i64, i64),
}

/// Returns `true` if the `password` character at the specified `position` matches
/// `letter`.
fn does_letter_match(letter: &char, password: &str, position: &i64) -> bool {
    password
        .chars()
        .nth((position - 1) as usize)
        .map(|password_char| password_char == *letter)
        .unwrap_or(false)
}
