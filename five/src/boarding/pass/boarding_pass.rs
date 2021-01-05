use crate::binary::space::{BinarySpace, BinarySpacePartition, NewBinarySpaceArgs};
use anyhow::{Context, Error, Result};

/// Represents information about a specific boarding pass.
#[derive(Debug)]
pub struct BoardingPass {
    seat_binary_space_string: String,
    seat_column: i64,
    seat_row: i64,
}

impl BoardingPass {
    pub fn from_seat_binary_space_string(seat_binary_space_string: &str) -> Result<BoardingPass> {
        let (column_binary_space_index, _) = seat_binary_space_string
            .char_indices()
            .nth(7)
            .with_context(|| format!(r#""{}" is not a valid seat id"#, seat_binary_space_string))?;

        let row_binary_space_partitions = BinarySpacePartition::list_from_row_binary_space_string(
            &seat_binary_space_string[..column_binary_space_index],
        )?;
        let row_binary_space = BinarySpace::new(NewBinarySpaceArgs {
            inclusive_start: 0,
            exclusive_end: 128,
        })?;
        let seat_row = row_binary_space
            .partition(row_binary_space_partitions)
            .evaluate()
            .context("Failed to partition row binary space")?;

        let column_binary_space_partitions =
            BinarySpacePartition::list_from_column_binary_space_string(
                &seat_binary_space_string[column_binary_space_index..],
            )?;
        let column_binary_space = BinarySpace::new(NewBinarySpaceArgs {
            inclusive_start: 0,
            exclusive_end: 8,
        })?;
        let seat_column = column_binary_space
            .partition(column_binary_space_partitions)
            .evaluate()
            .context("Failed to partition column binary space")?;

        Ok(BoardingPass {
            seat_binary_space_string: seat_binary_space_string.to_owned(),
            seat_column,
            seat_row,
        })
    }

    pub fn seat_id(&self) -> i64 {
        (self.seat_row * 8) + self.seat_column
    }
}

impl BinarySpacePartition {
    fn list_from_column_binary_space_string(
        column_binary_space_string: &str,
    ) -> Result<Vec<BinarySpacePartition>> {
        column_binary_space_string
            .chars()
            .map(|row_binary_space_char| match row_binary_space_char {
                'L' => Ok(BinarySpacePartition::Lower),
                'R' => Ok(BinarySpacePartition::Upper),
                _ => Err(Error::msg(format!(
                    "'{}' is not a valid column binary space char",
                    row_binary_space_char
                ))),
            })
            .collect::<Result<Vec<BinarySpacePartition>>>()
    }

    fn list_from_row_binary_space_string(
        row_binary_space_string: &str,
    ) -> Result<Vec<BinarySpacePartition>> {
        row_binary_space_string
            .chars()
            .map(|row_binary_space_char| match row_binary_space_char {
                'B' => Ok(BinarySpacePartition::Upper),
                'F' => Ok(BinarySpacePartition::Lower),
                _ => Err(Error::msg(format!(
                    "'{}' is not a valid row binary space char",
                    row_binary_space_char
                ))),
            })
            .collect::<Result<Vec<BinarySpacePartition>>>()
    }
}
