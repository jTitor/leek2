/*!
 * Defines internal methods of PipelineBuilder.
 */
use super::{PipelineBuilder, RenderPassLayout,
	SubpassPipelineLayout};
use super::ShaderLoad;

use graphics::device::internal::pipeline::render_pipeline::{elements, layout};

use std::rc::Rc;

use failure::Error;
use gfx_hal::{Backend, self as hal, pso};

/**
 * Implements top-level internal methods of PipelineBuilder.
 */
pub trait PipelineBuilderInternal<B: hal::Backend> {
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

impl<'a, B: hal::Backend> PipelineBuilderInternal<B> for PipelineBuilder<'a, B> {
	fn build_render_pass(&self, render_pass_layout: &RenderPassLayout, device: Rc<&B::Device>) -> Result<elements::Pass<B>, Error> {
		//First check that the layout is valid.
		let _layout_valid = render_pass_layout.layout_valid()?;

		let render_pass_result = {
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

			Ok()
		};

		render_pass_result
	}

	fn build_subpass_pipeline(&self, subpass_layout: &SubpassPipelineLayout<B>, device: Rc<&B::Device>) -> Result<elements::SubpassPipeline<B>, Error> {
		//Subpass pipeline is placed in this
		//double block so the shader modules are unloaded
		//the moment they don't need to be used.

		//Generate the subpass' descriptor set layout
		//from provided bindings if it exists.
		let mut set_layout: Option<&B::DescriptorSetLayout> = None;
		if let Some(set_layout_binding_vec) = subpass_layout.set_layout_bindings {
			set_layout = device.create_descriptor_set_layout(set_layout_binding_vec.as_slice()
			);
		}

		//Next, the pipeline layout.
		//This comes from the
		//descriptor set layout and push block array.
		device.create_pipeline_layout(
			set_layout,
			subpass_layout.push_block_constants.as_slice(),
		);

		//Load the actual shader files here.
		let shader_map = self.init_shader_map(device, subpass_layout)?;

		let subpass_pipeline_result = {
			//Specify all the entry points used by this
			//render subpass.
			let shader_set = self.shader_map_to_shader_set(shader_map)?;

			//Specify the pipeline's subpass
			//and connect it to the current render pass
			//TODO: Get render pass reference by
			//indexing into builder's renderpass
			//list
			//(subpass_layout.required_info.render_pass_index)
			let render_pass = ;
			let subpass = pass::Subpass { index: subpass_layout.required_info.subpass_index, main_pass: &render_pass };
			unimplemented!();

			//Create the pipeline description!
			let mut pipeline_desc = pso::GraphicsPipelineDesc::<B>::new(...);
			unimplemented!();

			//That gives the basic behavior of
			//the pipeline, but now we need
			//to provide all the linking details.
			//
			//How do we blend new values?
			//pipeline_desc.blender.targets.push(PipelineBlendDescription(...));
			for blend_desc in subpass_layout.blending_target_descriptions {
				pipeline_desc.blender.targets.push(blend_desc);
			}

			//What's the buffer layout for
			//vertex data?
			//pipeline_desc.vertex_buffers.push(BufferDescription {
			// 	stride: mem::size_of::<Vertex>() as u32,
			// 	rate: 0,
			// });
			for buffer_desc in subpass_layout.vertex_buffer_descriptions {
				pipeline_desc.vertex_buffers.push(buffer_desc);
			}

			//What attributes are available?
			//pipeline_desc.attributes.push(pso::AttributeDesc);
			//(attributes must be pushed
			//one at a time...)
			for attribute_desc in subpass_layout.attribute_descriptions {
				pipeline_desc.attributes.push(attribute_desc);
			}

			//Generate the final pipeline state object (PSO)
			//here.
			let pipeline_impl = device.create_graphics_pipeline(&pipeline_desc);

			Ok(SubpassPipeline::<B> {
				set_layout: set_layout,
				pipeline_layout: pipeline_layout,
				subpass: Subpass<'a, B>,
				pipeline_impl: pipeline_impl,
				resources_destroyed: false
			})
		};

		//The PSO is built, so we don't need the
		//shader modules anymore
		self.unload_shader_map(device, shader_map);

		subpass_pipeline_result
	}
}