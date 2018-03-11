/*!
 * Defines the BufferElement struct.
 */
use gfx_hal::{self as hal, pso};

/**
 * Describes the internal structure of a
 * vertex buffer element.
 */
pub type BufferElement<B: hal::Backend> = pso::Element<B>;