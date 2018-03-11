/*!
 * Defines the Subpass struct.
 */
use gfx_hal::{self as hal, pass};

/**
 * Encapsulates a distinct rendering operation
 * within a nodes::Pass.
 * 
 * Subpasses are grouped into Passes so
 * the backend can reorder them for optimal
 * performance.
 */
pub type Subpass<'a, B: hal::Backend> = pass::Subpass<'a, B>;