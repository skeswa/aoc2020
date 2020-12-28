extern crate anyhow;
extern crate tokio;

use anyhow::{Context, Error, Result};
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let input_file = read_input_file()
        .await
        .context("Failed to read input file")?;

    println!("Hello, world! {}", input_file);

    Ok(())
}

async fn read_input_file() -> Result<usize, Error> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let input_file_path = pwd.join("files/input.txt");

    let mut input_file = File::open(&input_file_path).await.context(format!(
        "Failed to open file at path \"{}\"",
        input_file_path.display()
    ))?;
    let mut input_file_contents = vec![];

    input_file
        .read_to_end(&mut input_file_contents)
        .await
        .context(format!(
            "Failed to read file at path \"{}\"",
            input_file_path.display()
        ))?;

    Ok(input_file_contents.len())
}
