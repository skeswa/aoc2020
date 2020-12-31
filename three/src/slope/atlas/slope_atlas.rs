use super::slope_contour::SlopeContour;
use anyhow::{Context, Result};

/// Maps out the spatial make up of a slope.
#[derive(Debug)]
pub struct SlopeAtlas {
    /// Each contour of the slope sorted in descending order by elevation.
    contours: Vec<SlopeContour>,
}

impl SlopeAtlas {
    pub fn from_file(slope_atlas_file_contents: &str) -> Result<SlopeAtlas> {
        let contours = slope_atlas_file_contents
            .lines()
            .map(|line| SlopeContour::from_text(line))
            .collect::<Result<Vec<SlopeContour>>>()
            .context("Failed to read contours")?;

        Ok(SlopeAtlas { contours })
    }

    pub fn height(&self) -> usize {
        self.contours.len()
    }
}
