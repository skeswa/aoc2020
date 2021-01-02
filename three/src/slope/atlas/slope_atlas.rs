use super::slope_contour::SlopeContour;
use super::slope_feature::SlopeFeature;
use anyhow::{Context, Error, Result};

/// Maps out the spatial make up of a slope.
#[derive(Debug)]
pub struct SlopeAtlas {
    /// How wide each contour is.
    pub breadth: i64,
    /// How many contours there are.
    pub height: i64,

    /// Each contour of the slope sorted in descending order by elevation.
    contours: Vec<SlopeContour>,
}

impl SlopeAtlas {
    /// Constructs a new `SlopeAtlas` based on the provided
    /// `slope_atlas_file_contents`.
    ///
    /// For more on the format of `slope_atlas_file_contents`,
    /// see https://adventofcode.com/2020/day/3.
    pub fn from_file(slope_atlas_file_contents: &str) -> Result<SlopeAtlas> {
        let contours = slope_atlas_file_contents
            .lines()
            .map(|line| SlopeContour::from_text(line))
            .collect::<Result<Vec<SlopeContour>>>()
            .context("Failed to read contours")?;

        let height = contours.len() as i64;
        if height < 1 {
            return Err(Error::msg("Slope atlas file has no contours"));
        }

        let breadth = contours.get(0).unwrap().breadth();
        let contour_with_different_breadth = contours
            .iter()
            .enumerate()
            .find(|(_i, contour)| contour.breadth() != breadth);
        if contour_with_different_breadth.is_some() {
            return Err(Error::msg(format!(
                "Contour at index {} has an incosistent breadth ({})",
                contour_with_different_breadth.unwrap().0,
                breadth
            )));
        }

        Ok(SlopeAtlas {
            breadth,
            contours,
            height,
        })
    }

    /// Returns the feature at the specified `position`.
    pub fn feature_at(&self, position: (i64, i64)) -> Option<SlopeFeature> {
        let (x_position, y_position) = position;

        self.contours.get(y_position as usize).and_then(|contour| {
            let unaligned_feature_index = x_position % self.breadth;
            let feature_index = if unaligned_feature_index < 0 {
                self.breadth + unaligned_feature_index
            } else {
                unaligned_feature_index
            };

            contour.feature_at(feature_index)
        })
    }
}
