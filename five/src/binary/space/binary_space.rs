use anyhow::{Error, Result};

/// A range of integers that can be arbitrarily split in half.
#[derive(Debug)]
pub struct BinarySpace {
    range: i64,
    start: i64,
}

/// Enumerates every kind of `BinarySpace` partition.
#[derive(Debug)]
pub enum BinarySpacePartition {
    Lower,
    Upper,
}

/// Arguments for `BinarySpace::new(...)`.
pub struct NewBinarySpaceArgs {
    /// First index included in the resulting new binary space.
    pub inclusive_start: i64,
    /// First index not included in the resulting new binary space.
    pub exclusive_end: i64,
}

impl BinarySpace {
    /// Creates a new instance of `BinarySpace`.
    ///
    /// Returns `Err(...)` if `exclusive_end` is than or equal
    /// `inclusive_start`.
    pub fn new(
        NewBinarySpaceArgs {
            inclusive_start,
            exclusive_end,
        }: NewBinarySpaceArgs,
    ) -> Result<BinarySpace> {
        if exclusive_end > inclusive_start {
            Ok(BinarySpace {
                start: inclusive_start,
                range: exclusive_end - inclusive_start,
            })
        } else {
            Err(Error::msg(format!(
                "[{}, {}) is not a valid range",
                inclusive_start, exclusive_end
            )))
        }
    }

    /// If this binary space has just one number in its range, that number is
    /// returned.
    pub fn evaluate(&self) -> Result<i64> {
        if self.range == 1 {
            Ok(self.start)
        } else {
            Err(Error::msg(format!(
                "Range of binary space is still too broad ({})",
                self.range
            )))
        }
    }

    /// Splits this binary space in half for each partition specified by
    /// `partitions`.
    pub fn partition(mut self, partitions: Vec<BinarySpacePartition>) -> Self {
        for partition in partitions {
            if self.range < 2 {
                break;
            }

            let half_range = (self.range as f64) / 2.0;

            match partition {
                BinarySpacePartition::Lower => {
                    self.range = half_range.floor() as i64;
                }
                BinarySpacePartition::Upper => {
                    let delta = half_range.ceil() as i64;

                    self.start += delta;
                    self.range -= delta;
                }
            }
        }

        self
    }
}
