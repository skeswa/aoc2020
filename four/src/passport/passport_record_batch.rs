use super::passport_record::PassportRecord;
use anyhow::{Context, Result};

/// A collection of passport records.
#[derive(Debug)]
pub struct PassportRecordBatch {
    records: Vec<PassportRecord>,
}

impl PassportRecordBatch {
    pub fn from_file(passport_record_batch_file_contents: &str) -> Result<PassportRecordBatch> {
        let records = passport_record_batch_file_contents
            .split("\n\n")
            .map(|passport_record_text| PassportRecord::from_text(passport_record_text))
            .collect::<Result<Vec<PassportRecord>>>()
            .context("Failed to parse passport records")?;

        Ok(PassportRecordBatch { records })
    }

    pub fn number_of_records(&self) -> usize {
        self.records.len()
    }

    pub fn number_of_valid_records(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.is_valid())
            .count()
    }
}
