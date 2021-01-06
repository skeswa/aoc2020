extern crate anyhow;
extern crate tokio;

mod binary;
mod boarding;

use anyhow::{Context, Result};
use boarding::pass::BoardingPass;
use std::collections::HashSet;
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let boarding_passes = read_boarding_passes()
        .await
        .context("Failed to read boarding passes")?;

    let max_boarding_pass_seat_id = boarding_passes
        .iter()
        .map(|boarding_pass| boarding_pass.seat_id)
        .max()
        .context("Failed to order boarding passes by seat id")?;

    println!(
        "Maximum boarding pass seat id: {}",
        max_boarding_pass_seat_id
    );

    let boarding_pass_seat_ids = boarding_passes
        .iter()
        .map(|boarding_pass| boarding_pass.seat_id)
        .collect::<HashSet<i64>>();

    println!("Look for your seat in the plane map below:");
    for seat_row in 0..128 {
        print!("{}\t", seat_row);

        for seat_column in 0..8 {
            let seat_id = BoardingPass::calculate_seat_id(seat_row, seat_column);

            if boarding_pass_seat_ids.contains(&seat_id) {
                print!("X");
            } else {
                print!("?");
            }
        }

        println!();
    }

    Ok(())
}

/// Turns input file into a new list of boarding passes.
async fn read_boarding_passes() -> Result<Vec<BoardingPass>> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let boarding_passes_file_path = pwd.join("files/input.txt");

    let mut boarding_passes_file =
        File::open(&boarding_passes_file_path)
            .await
            .with_context(|| {
                format!(
                    "Failed to open file at path \"{}\"",
                    boarding_passes_file_path.display()
                )
            })?;
    let mut raw_boarding_passes_file_contents = vec![];

    boarding_passes_file
        .read_to_end(&mut raw_boarding_passes_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                boarding_passes_file_path.display()
            )
        })?;

    let boarding_passes_file_contents = String::from_utf8_lossy(&raw_boarding_passes_file_contents);

    boarding_passes_file_contents
        .lines()
        .map(|seat_binary_space_string| {
            BoardingPass::from_seat_binary_space_string(seat_binary_space_string)
        })
        .collect::<Result<Vec<BoardingPass>>>()
}
