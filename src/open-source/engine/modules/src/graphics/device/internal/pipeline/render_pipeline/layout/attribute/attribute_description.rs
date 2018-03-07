/*!
 * Defines the AttributeDescription struct.
 */
use gfx_hal::pso;

/**
 * Describes the layout of elements within
 * a vertex buffer, but does not
 * specify the internal structure of those elements.
 * 
 * Stride is the byte distance between two
 * elements, while Rate is the number of elements between
 * attributes. Rate can be set to 0 if you're not using
 * instancing.
 */
pub type AttributeDescription = pso::AttributeDesc;