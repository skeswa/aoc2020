extern crate anyhow;
extern crate tokio;

use anyhow::{Context, Error, Result};
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let expense_report_entries = read_expense_report_entries()
        .await
        .context("Failed to read expense report")?;

    let matching_expense_report_entries =
        find_matching_expense_report_entries(expense_report_entries)
            .context("Could not find matching expense report entries")?;

    println!(
        "Entry 1:\t{}\nEntry 2:\t{}\nProduct:\t{}",
        matching_expense_report_entries.0,
        matching_expense_report_entries.1,
        matching_expense_report_entries.0 * matching_expense_report_entries.1
    );

    Ok(())
}

/// Finds a pair of expense report enties that sum to `2020`.
fn find_matching_expense_report_entries(expense_report_entries: Vec<i32>) -> Option<(i32, i32)> {
    expense_report_entries
        .iter()
        .flat_map(|expense_report_entry| {
            expense_report_entries
                .iter()
                .map(move |other_expense_report_entry| {
                    (expense_report_entry, other_expense_report_entry)
                })
        })
        .find(|(expense_report_entry, other_expense_report_entry)| {
            *expense_report_entry + *other_expense_report_entry == 2020
        })
        .map(|(a, b)| (*a, *b))
}

/// Reads the input file, returning each line represented as a 32-bit integer.
async fn read_expense_report_entries() -> Result<Vec<i32>, Error> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let expense_report_file_path = pwd.join("files/input.txt");

    let mut expense_report_file =
        File::open(&expense_report_file_path)
            .await
            .with_context(|| {
                format!(
                    "Failed to open file at path \"{}\"",
                    expense_report_file_path.display()
                )
            })?;
    let mut expense_report_file_contents = vec![];

    expense_report_file
        .read_to_end(&mut expense_report_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                expense_report_file_path.display()
            )
        })?;

    String::from_utf8_lossy(&expense_report_file_contents)
        .lines()
        .enumerate()
        .map(
            |(expense_report_file_line_index, expense_report_file_line_text)| {
                expense_report_file_line_text
                    .parse::<i32>()
                    .with_context(|| {
                        format!(
                    "Failed to convert expense report file entry at line {} (\"{}\") to integer",
                    /* lineNumber= */ expense_report_file_line_index + 1,
                    expense_report_file_line_text
                )
                    })
            },
        )
        .collect()
}
