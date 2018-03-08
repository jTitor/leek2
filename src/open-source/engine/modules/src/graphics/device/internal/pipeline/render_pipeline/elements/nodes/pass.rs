/*!
 * Defines the RenderPass struct.
 */
use gfx_hal as hal;
use gfx_hal::RenderPass;

/**
 * Encapsulates a top-level rendering pass.
 * Contains at least one Subpass, which can only be
 * run within this pass.
 */
pub type Pass<B: hal::Backend> = B::RenderPass;