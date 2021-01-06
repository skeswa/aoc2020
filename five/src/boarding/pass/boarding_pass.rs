use crate::binary::space::{BinarySpace, BinarySpacePartition, NewBinarySpaceArgs};
use anyhow::{Context, Error, Result};

/// Represents information about a specific boarding pass.
#[derive(Debug)]
pub struct BoardingPass {
    /// Unique identified for the seat associated with this boarding pass; this
    /// value is a pure function of `seat_column` and `seat_row`.
    pub seat_id: i64,

    /// `String` describing the binary space used to calculate both
    /// `seat_column` and `seat_row`.
    seat_binary_space_string: String,

    /// Column index of the seat associated with this boarding pass.
    seat_column: i64,

    /// Row index of the seat associated with this boarding pass.
    seat_row: i64,
}

impl BoardingPass {
    /// Returns a new `BoardingPass` built by parsing the given
    /// `seat_binary_space_string`.
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

        let seat_id = BoardingPass::calculate_seat_id(seat_row, seat_column);

        Ok(BoardingPass {
            seat_binary_space_string: seat_binary_space_string.to_owned(),
            seat_column,
            seat_id,
            seat_row,
        })
    }

    /// Combines `seat_row` and `seat_column` into a unique seat identifier.
    pub fn calculate_seat_id(seat_row: i64, seat_column: i64) -> i64 {
        (seat_row * 8) + seat_column
    }
}

impl BinarySpacePartition {
    /// Maps the individual characters of the given
    /// `column_binary_space_string` to a sequence of binary space partitions.
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

    /// Maps the individual characters of the given
    /// `row_binary_space_string` to a sequence of binary space partitions.
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
