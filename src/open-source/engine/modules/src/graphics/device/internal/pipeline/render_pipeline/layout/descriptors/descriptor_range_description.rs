/*!
 * Defines the DescriptorRangeDescription struct.
 */
use gfx_hal::pso;

/**
 * Defines the maximum number of a given type of
 * descriptor that can exist in a given descriptor pool.
 */
pub type DescriptorRangeDescription = pso::DescriptorRangeDesc;
