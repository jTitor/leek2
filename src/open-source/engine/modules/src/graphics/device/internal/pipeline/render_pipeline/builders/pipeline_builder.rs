/*!
 * Defines the RenderPipelineBuilder struct.
 */
use graphics::device::internal::pipeline::Pipeline;

use std::marker::PhantomData;

use failure::Error;
use gfx_hal as hal;

/**
 * Basic builder struct for a render pipeline.
 */
pub struct PipelineBuilder<B: hal::Backend> {
	_backend_type: PhantomData<B>
}

impl<B: hal::Backend> PipelineBuilder<B> {
	/**
	 * Builds the RenderPipeline if possible.
	 */
	pub fn build(&self, device: &mut B::Device) -> Result<Pipeline<B>, Error> {
		unimplemented!();
		//Perform all the necessary calls to
		//generate the pipeline's individual elements:
		
		//Generate the descriptor set layout from
		//provided bindings.
		//(device.create_*())
		unimplemented!();

		//Next, the push block constants.

		//Pipeline layout is generated from
		//descriptor set layout and push block array.
		//device.create_descriptor_set_layout(&[...])
		unimplemented!();

		//Generate each render pass:
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

		//Subpass pipeline is placed in this
		//double block so the shader modules are unloaded
		//the moment they don't need to be used.
		let subpass_pipeline = {
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

			//subpass_pipeline
		};

		//Create external-facing descriptors
		//the render calls can bind to.
		// Descriptor pool -
		// this describes how many descriptors
		// can be allocated at any given time
		// and in how many sets of the given layout.
		//let mut desc_pool = device.create_descriptor_pool(1, &[pso::DescriptorRangeDesc, ...]);
		//let desc_set = desc_pool.allocate_set(&set_layout);
		//TODO_rust: make these part of result struct
		unimplemented!();

		//Put all of the subpass pipelines in a vec...
		unimplemented!();

		//along with the render passes...
		unimplemented!();

		//And the full pipeline is ready.
		//let pipeline = ...
		unimplemented!();

		//Ok(pipeline)
	}
}