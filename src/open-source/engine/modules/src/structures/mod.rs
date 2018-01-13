/*!
 * Stores data and spatial subdivision structures
 * that are type-agnostic.
 */

pub mod tree;
pub use structures::tree::{Tree, TreeOps};

pub mod octree;
pub use structures::octree::{OcTree, OcTreeOps};