/*!
 * Defines the SubpassDescription struct.
 */
use gfx_hal::pass;

/**
 * Describes the internal layout of
 * a subpass.
 */
pub type SubpassDescription<'a> = pass::SubpassDesc<'a>;