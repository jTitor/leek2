/*!
 * Contains modules defining all the different parts
 * that make a pipeline graph.
 */
pub mod nodes;

pub mod links;

mod descriptor_set;
pub use self::descriptor_set::DescriptorSet;