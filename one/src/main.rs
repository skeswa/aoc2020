extern crate anyhow;
extern crate tokio;

use anyhow::{Context, Error, Result};
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let input_integers = read_expense_report_entries()
        .await
        .context("Failed to read input file")?;

    println!("Hello, world! {:?}", input_integers);

    Ok(())
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
