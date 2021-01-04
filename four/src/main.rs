extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod passport;

use anyhow::{Context, Error, Result};
use passport::PassportRecordBatch;
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let passport_record_batch = read_passport_record_batch()
        .await
        .context("Failed to read passport batch")?;

    println!(
        "Complete passport records: {} / {}",
        passport_record_batch.number_of_complete_records(),
        passport_record_batch.number_of_records()
    );

    println!(
        "Valid passport records: {} / {}",
        passport_record_batch.number_of_valid_records(),
        passport_record_batch.number_of_records()
    );

    Ok(())
}

/// Turns input file into a new instance of `PassportRecordBatch`.
async fn read_passport_record_batch() -> Result<PassportRecordBatch, Error> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let passport_record_batch_file_path = pwd.join("files/input.txt");

    let mut passport_record_batch_file = File::open(&passport_record_batch_file_path)
        .await
        .with_context(|| {
            format!(
                "Failed to open file at path \"{}\"",
                passport_record_batch_file_path.display()
            )
        })?;
    let mut raw_passport_record_batch_file_contents = vec![];

    passport_record_batch_file
        .read_to_end(&mut raw_passport_record_batch_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                passport_record_batch_file_path.display()
            )
        })?;

    let passport_record_batch_file_contents =
        String::from_utf8_lossy(&raw_passport_record_batch_file_contents);

    PassportRecordBatch::from_file(&passport_record_batch_file_contents)
}
