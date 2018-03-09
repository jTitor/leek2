/*!
 * Defines the Subpass struct.
 */
use gfx_hal::pass;

/**
 * Encapsulates a distinct rendering operation
 * within a nodes::Pass.
 * 
 * Subpasses are grouped into Passes so
 * the backend can reorder them for optimal
 * performance.
 */
pub type Subpass<'a> = pass::Subpass<'a, B>;