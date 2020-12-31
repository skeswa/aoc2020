extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod password_database;

use anyhow::{Context, Error, Result};
use password_database::PasswordDatabase;
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let password_database = read_password_database()
        .await
        .context("Failed to read password database")?;

    let number_of_valid_password_entries = password_database
        .entries
        .iter()
        .filter(|password_database_entry| password_database_entry.is_valid())
        .count();

    println!(
        "Valid password entries: {} / {}",
        number_of_valid_password_entries,
        password_database.entries.len()
    );

    Ok(())
}

// /// Finds a pair of expense report enties that sum to `2020`.
// fn find_expense_report_entry_pair(expense_report_entries: &Vec<i32>) -> Option<(i32, i32)> {
//     expense_report_entries
//         .iter()
//         .flat_map(|expense_report_entry| {
//             expense_report_entries
//                 .iter()
//                 .map(move |other_expense_report_entry| {
//                     (expense_report_entry, other_expense_report_entry)
//                 })
//         })
//         .find(|(expense_report_entry, other_expense_report_entry)| {
//             *expense_report_entry + *other_expense_report_entry == 2020
//         })
//         .map(|(a, b)| (*a, *b))
// }

// /// Finds a trio of expense report enties that sum to `2020`.
// fn find_expense_report_entry_trio(expense_report_entries: &Vec<i32>) -> Option<(i32, i32, i32)> {
//     expense_report_entries
//         .iter()
//         .flat_map(|expense_report_entry| {
//             expense_report_entries
//                 .iter()
//                 .flat_map(move |other_expense_report_entry| {
//                     expense_report_entries
//                         .iter()
//                         .map(move |another_expense_report_entry| {
//                             (
//                                 expense_report_entry,
//                                 other_expense_report_entry,
//                                 another_expense_report_entry,
//                             )
//                         })
//                 })
//         })
//         .find(
//             |(expense_report_entry, other_expense_report_entry, another_expense_report_entry)| {
//                 *expense_report_entry + *other_expense_report_entry + *another_expense_report_entry
//                     == 2020
//             },
//         )
//         .map(|(a, b, c)| (*a, *b, *c))
// }

/// Reads the input file, returning each line represented as a 32-bit integer.
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
