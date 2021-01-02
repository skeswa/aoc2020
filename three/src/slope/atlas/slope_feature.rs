use anyhow::{Error, Result};
/// Enumerates every kind of thing that can appear on the slope.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SlopeFeature {
    /// Describes a part of the slope with nothing on it.
    Nothing,
    /// Describes a tree on the slope.
    Tree,
}

impl SlopeFeature {
    pub fn from_char(slope_feature_char: char) -> Result<SlopeFeature> {
        match slope_feature_char {
            '#' => Ok(SlopeFeature::Tree),
            '.' => Ok(SlopeFeature::Nothing),
            _ => Err(Error::msg(format!(
                "Found unrecognized character '{}'",
                slope_feature_char
            ))),
        }
    }
}
