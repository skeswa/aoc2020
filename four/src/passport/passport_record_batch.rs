use super::passport_record::PassportRecord;
use super::validated_passport_record::ValidatedPassportRecord;
use anyhow::{Context, Result};

/// A collection of passport records.
#[derive(Debug)]
pub struct PassportRecordBatch {
    /// All unvalidated passport records in the batch.
    records: Vec<PassportRecord>,
}

impl PassportRecordBatch {
    /// Processes and returns the batch of passport records enclosed within
    /// the given `passport_record_batch_file_contents`.
    pub fn from_file(passport_record_batch_file_contents: &str) -> Result<PassportRecordBatch> {
        let records = passport_record_batch_file_contents
            .split("\n\n")
            .map(|passport_record_text| PassportRecord::from_string(passport_record_text))
            .collect::<Result<Vec<PassportRecord>>>()
            .context("Failed to parse passport records")?;

        Ok(PassportRecordBatch { records })
    }

    /// Returns the total number of passport records in this batch.
    pub fn number_of_records(&self) -> usize {
        self.records.len()
    }

    /// Returns the number of passport records in this batch that have all of
    /// the requisite attributes.
    pub fn number_of_complete_records(&self) -> usize {
        self.records
            .iter()
            .filter(|passport_record| passport_record.is_complete())
            .count()
    }

    /// Returns the number of passport records in this batch that have all of
    /// the requisite attributes and valid values for each.
    pub fn number_of_valid_records(&self) -> usize {
        self.records
            .iter()
            .filter(|passport_record| {
                ValidatedPassportRecord::from_passport_record(passport_record).is_ok()
            })
            .count()
    }
}
