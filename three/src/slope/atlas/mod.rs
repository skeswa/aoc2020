mod slope_atlas;
mod slope_contour;
mod slope_feature;

/// Maps out the spatial make up of a slope.
pub type SlopeAtlas = slope_atlas::SlopeAtlas;

/// Enumerates every kind of thing that can appear on the slope.
pub type SlopeFeature = slope_feature::SlopeFeature;
