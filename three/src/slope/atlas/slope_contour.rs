use super::slope_feature::SlopeFeature;
use anyhow::{Context, Result};

/// Represents one elevation of the slope.
#[derive(Debug)]
pub struct SlopeContour {
    /// Each feature of this contour of the slope, ordered from left to right.
    features: Vec<SlopeFeature>,
}

impl SlopeContour {
    /// Creates a new `SlopeContour` using the text from a slope atlas file.
    pub fn from_text(text: &str) -> Result<SlopeContour> {
        let features = text
            .chars()
            .map(|text_char| SlopeFeature::from_char(text_char))
            .collect::<Result<Vec<SlopeFeature>>>()
            .context("Failed to parse slop contours")?;

        Ok(SlopeContour { features })
    }

    /// How wide this contour is.
    pub fn breadth(&self) -> i64 {
        self.features.len() as i64
    }

    /// Returns the feature at the specified `feature_index`.
    pub fn feature_at(&self, feature_index: i64) -> Option<SlopeFeature> {
        self.features
            .get(feature_index as usize)
            .map(|feature| *feature)
    }
}
