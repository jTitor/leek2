/*!
 * Defines the RenderPassLayout struct.
 */
use gfx_hal::{self as hal, image as i, pass, pso}

/**
 * Defines the structure of a Pass for
 * a PipelineBuilder.
 */
#[derive(Debug)]
pub struct RenderPassLayout {
	pub attachments: Vec<pass::Attachment>,
	pub subpass_descriptions: Vec<pass::SubpassDesc>,
	pub dependencies: Vec<pass::SubpassDependency>
}

impl RenderPassLayout {
	fn new() -> RenderPassLayout {
		RenderPassLayout::default()
	}
}