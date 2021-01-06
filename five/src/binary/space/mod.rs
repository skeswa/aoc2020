mod binary_space;

/// A range of integers that can be arbitrarily split in half.
pub type BinarySpace = binary_space::BinarySpace;

/// Enumerates every kind of `BinarySpace` partition.
pub type BinarySpacePartition = binary_space::BinarySpacePartition;

/// Arguments for `BinarySpace::new(...)`.
pub type NewBinarySpaceArgs = binary_space::NewBinarySpaceArgs;
