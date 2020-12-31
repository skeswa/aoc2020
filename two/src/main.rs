extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod password_database;

use anyhow::{Context, Error, Result};
use password_database::{PasswordDatabase, PasswordValidationStrategy};
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let password_database = read_password_database()
        .await
        .context("Failed to read password database")?;

    let number_of_valid_repetition_range_password_entries = password_database
        .entries
        .iter()
        .filter(|password_database_entry| {
            password_database_entry.is_valid(PasswordValidationStrategy::LetterRepetitionRange)
        })
        .count();

    println!(
        "Valid repetition range password entries: {} / {}",
        number_of_valid_repetition_range_password_entries,
        password_database.entries.len()
    );

    let number_of_valid_letter_positions_password_entries = password_database
        .entries
        .iter()
        .filter(|password_database_entry| {
            password_database_entry.is_valid(PasswordValidationStrategy::LetterPositions)
        })
        .count();

    println!(
        "Valid letter position password entries: {} / {}",
        number_of_valid_letter_positions_password_entries,
        password_database.entries.len()
    );

    Ok(())
}

/// Turns input file into a new instance of `PasswordDatabase`.
async fn read_password_database() -> Result<PasswordDatabase, Error> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let password_database_file_path = pwd.join("files/input.txt");

    let mut password_database_file = File::open(&password_database_file_path)
        .await
        .with_context(|| {
            format!(
                "Failed to open file at path \"{}\"",
                password_database_file_path.display()
            )
        })?;
    let mut raw_password_database_file_contents = vec![];

    password_database_file
        .read_to_end(&mut raw_password_database_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                password_database_file_path.display()
            )
        })?;

    let password_database_file_contents =
        String::from_utf8_lossy(&raw_password_database_file_contents);

    PasswordDatabase::from_file(&password_database_file_contents)
}
