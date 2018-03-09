/*!
 * Defines internal methods of PipelineBuilder.
 */
use super::{PipelineBuilder, RenderPassLayout,
	SubpassPipelineLayout};

use graphics::device::internal::pipeline::render_pipeline::{elements, layout};

use std::rc::Rc;

use failure::Error;
use gfx_hal as hal;
use gfx_hal::pso;

/**
 * Implements internal methods of PipelineBuilder.
 */
pub trait PipelineBuilderInternal<B: hal::Backend> {
	/**
	 * Builds the descriptor set layout
	 * used to generate the final pipeline's
	 * descriptor set.
	 * 
	 * TODO: The return value is really gross
	 * looking, might move this directly into
	 * PipelineBuilder::build()
	 */
	fn build_descriptor_set_layout<D>(&self, device: Rc<&B::Device>) -> Result<D, Error> where D: B::DescriptorSetLayout;

	/**
	 * Builds a render pass
	 * with the given device.
	 */
	fn build_render_pass(&self, render_pass_layout: &RenderPassLayout, device: Rc<&B::Device>) -> Result<elements::Pass<B>, Error>;

	/**
	 * Builds a subpass pipeline
	 * with the given device.
	 */
	fn build_subpass_pipeline(&self, subpass_layout: &SubpassPipelineLayout<B>, device: Rc<&B::Device>) -> Result<elements::SubpassPipeline<B>, Error>;
}

impl<B: hal::Backend> PipelineBuilderInternal<B> for PipelineBuilder<B> {
	fn build_descriptor_set_layout<D>(&self, device: Rc<&B::Device>) -> Result<D, Error> where D: pso::DescriptorSetLayout {
		//Generate the descriptor set layout from
		//provided bindings.
		//(device.create_*())
		unimplemented!();

		//Next, the push block constants.

		//Pipeline layout is generated from
		//descriptor set layout and push block array.
		//device.create_descriptor_set_layout(&[...])
		unimplemented!();

		Ok(())
	}

	fn build_render_pass(&self, render_pass_layout: &RenderPassLayout, device: Rc<&B::Device>) -> Result<elements::Pass<B>, Error> {
		//First check that the layout is valid.
		let _layout_valid = render_pass_layout.layout_valid()?;

		let render_pass = {
			//Describe attachments here.
			//attachment = ...
			unimplemented!();

			//List the what attachments each subpass
			//has access to.
			//subpass_desc = ...
			unimplemented!();

			//Link the subpass to this render pass,
			//link them together as needed
			//let dependency = ...
			unimplemented!();

			//And actually create the render pass here.
			//device.create_render_pass(&[attachment], &[subpass_desc], &[dependency])
			unimplemented!();
		};

		Ok(render_pass)
	}

	fn build_subpass_pipeline(&self, subpass_layout: &SubpassPipelineLayout<B>,device: Rc<&B::Device>) -> Result<elements::SubpassPipeline<B>, Error> {
		//Subpass pipeline is placed in this
		//double block so the shader modules are unloaded
		//the moment they don't need to be used.

		//Load the actual shader files here.
		unimplemented!();
		// let vs_module = device
		// 	.create_shader_module(include_bytes!("data/vert.spv"))
		// 	.unwrap();
		// let fs_module = device
		// 	.create_shader_module(include_bytes!("data/frag.spv"))
		// 	.unwrap();
		unimplemented!();
		//const ENTRY_NAME: &str = "TODO";

		let subpass_pipeline = {
			//Specify the entry points
			//for the shader types:
			//let (vs_entry, fs_entry) = (ShaderEntryPoint::<B>, ...);
			unimplemented!();

			//Specify all the entry points used by this
			//render subpass.
			//let shader_entries = GraphicsShaderSet { ... };
			unimplemented!();

			//Specify the pipeline's subpass
			//and connect it to the current render pass
			//let subpass = pass::Subpass { index: ?, main_pass: &render_pass };
			unimplemented!();

			//Create the pipeline description!
			//let mut pipeline_desc = GraphicsPipelineDesc::new(...);
			unimplemented!();

			//That gives the basic behavior of
			//the pipeline, but now we need
			//to provide all the linking details.
			//
			//How do we blend new values?
			//pipeline_desc.blender.targets.push(PipelineBlendDescription(...));
			unimplemented!();

			//What's the buffer layout for
			//vertex data?
			//pipeline_desc.vertex_buffers.push(BufferDescription {
			// 	stride: mem::size_of::<Vertex>() as u32,
			// 	rate: 0,
			// });
			unimplemented!();

			//What attributes are available?
			//pipeline_desc.attributes.push(pso::AttributeDesc);
			//(attributes must be pushed
			//one at a time...)
			unimplemented!();

			//Generate the final pipeline state object (PSO)
			//here.
			//device.create_graphics_pipeline(&pipeline_desc)
			unimplemented!();
		};

		//The PSO is built, so we don't need the
		//shader modules anymore
		//device.destroy_shader_module(vs_module);
		//device.destroy_shader_module(fs_module);
		unimplemented!();

		Ok(subpass_pipeline)
	}
}