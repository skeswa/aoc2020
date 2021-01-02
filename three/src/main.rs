extern crate anyhow;

mod slope;
mod toboggan;

use anyhow::{Context, Error, Result};
use slope::atlas::{SlopeAtlas, SlopeFeature};
use std::env::current_dir;
use toboggan::trajectory::TobogganTrajectory;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let slope_atlas = read_slope_atlas()
        .await
        .context("Failed to read slope atlas")?;

    let trajectories: Vec<TobogganTrajectory> = vec![
        TobogganTrajectory::new((0, 0), (1, 1)),
        TobogganTrajectory::new((0, 0), (3, 1)),
        TobogganTrajectory::new((0, 0), (5, 1)),
        TobogganTrajectory::new((0, 0), (7, 1)),
        TobogganTrajectory::new((0, 0), (1, 2)),
    ];

    let trajectory_tree_counts = trajectories
        .iter()
        .map(|trajectory| {
            (
                trajectory,
                number_of_trees_along_trajectory(&slope_atlas, trajectory),
            )
        })
        .collect::<Vec<(&TobogganTrajectory, usize)>>();

    for (trajectory, tree_count) in trajectory_tree_counts.iter() {
        println!("{}: {}", trajectory, tree_count);
    }

    let tree_count_product = trajectory_tree_counts
        .iter()
        .map(|(_trajectory, tree_count)| *tree_count)
        .product::<usize>();

    println!("\nTree count product: {}", tree_count_product);

    Ok(())
}

/// Calculates how many trees on the `slope_atlas` fall along the given
/// `trajectory`.
fn number_of_trees_along_trajectory(
    slope_atlas: &SlopeAtlas,
    trajectory: &TobogganTrajectory,
) -> usize {
    trajectory
        .descend(slope_atlas.height)
        .map(|position| {
            slope_atlas
                .feature_at(position)
                .unwrap_or(SlopeFeature::Nothing)
        })
        .filter(|feature| *feature == SlopeFeature::Tree)
        .count()
}

/// Turns input file into a new instance of `SlopeAtlas`.
async fn read_slope_atlas() -> Result<SlopeAtlas, Error> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let slope_atlas_file_path = pwd.join("files/input.txt");

    let mut slope_atlas_file = File::open(&slope_atlas_file_path).await.with_context(|| {
        format!(
            "Failed to open file at path \"{}\"",
            slope_atlas_file_path.display()
        )
    })?;
    let mut raw_slope_atlas_file_contents = vec![];

    slope_atlas_file
        .read_to_end(&mut raw_slope_atlas_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                slope_atlas_file_path.display()
            )
        })?;

    SlopeAtlas::from_file(&String::from_utf8_lossy(&raw_slope_atlas_file_contents))
}
