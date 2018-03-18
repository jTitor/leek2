/*!
 * Defines the RenderPassLayout struct.
 */
use gfx_hal::{pass};

use failure::Error;

/**
 * Defines the structure of a Pass for
 * a PipelineBuilder.
 */
#[derive(Debug, Default)]
pub struct RenderPassLayout<'a> {
	pub attachments: Vec<pass::Attachment>,
	pub subpass_descriptions: Vec<pass::SubpassDesc<'a>>,
	pub dependencies: Vec<pass::SubpassDependency>
}

impl<'a> RenderPassLayout<'a> {
	pub fn new() -> RenderPassLayout<'a> {
		RenderPassLayout::default()
	}

	pub fn layout_valid(&self) -> Result<(), Error> {
		if self.attachments.len() < 1 {
			return Err(RenderPassLayoutError::NotEnoughAttachments.into());
		}
		if self.subpass_descriptions.len() < 1 {
			return Err(RenderPassLayoutError::NotEnoughSubpassDescriptions.into());
		}
		if self.dependencies.len() < 1 {
			return Err(RenderPassLayoutError::NotEnoughDependencies.into());
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
