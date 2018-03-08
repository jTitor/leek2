/*!
 * Defines the RenderPassLayout struct.
 */
use gfx_hal::{self as hal, image as i, pass, pso}

use failure::Error;

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
	pub fn new() -> RenderPassLayout {
		RenderPassLayout::default()
	}

	pub fn layout_valid(&self) -> Result<(), Error> {
		if self.attachments.len() < 1 {
			return RenderPassLayoutError::NotEnoughAttachments;
		}
		if self.subpass_descriptions.len() < 1 {
			return RenderPassLayoutError::NotEnoughSubpassDescriptions;
		}
		if self.dependencies.len() < 1 {
			return RenderPassLayoutError::NotEnoughDependencies;
		}
		
		Ok(())
	}
}

#[derive(Debug, Fail)]
pub enum RenderPassLayoutError {
	#[fail(display = "RenderPassLayout.attachments needs at least one element")]
	NotEnoughAttachments,
	#[fail(display = "RenderPassLayout.subpass_descriptions needs at least one element")]
	NotEnoughSubpassDescriptions,
	#[fail(display = "RenderPassLayout.dependencies needs at least one element")]
	NotEnoughDependencies,
}
