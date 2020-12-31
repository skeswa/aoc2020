extern crate anyhow;

mod slope;

use anyhow::{Context, Error, Result};
use slope::atlas::SlopeAtlas;
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let slope_atlas = read_slope_atlas()
        .await
        .context("Failed to read slope atlas")?;

    println!("Hello, world! {:?}", slope_atlas.height());

    Ok(())
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
