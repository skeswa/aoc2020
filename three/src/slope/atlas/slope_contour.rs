use super::slope_feature::SlopeFeature;
use anyhow::{Context, Result};

/// Represents one elevation of the slope.
#[derive(Debug)]
pub struct SlopeContour {
    /// Each feature of this contour of the slope, ordered from left to right.
    features: Vec<SlopeFeature>,
}

impl SlopeContour {
    pub fn from_text(text: &str) -> Result<SlopeContour> {
        let features = text
            .chars()
            .map(|text_char| SlopeFeature::from_char(text_char))
            .collect::<Result<Vec<SlopeFeature>>>()
            .context("Failed to parse slop contours")?;

        Ok(SlopeContour { features })
    }
}
