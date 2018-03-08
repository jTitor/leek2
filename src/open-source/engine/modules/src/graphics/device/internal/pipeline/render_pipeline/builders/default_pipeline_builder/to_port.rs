/*!
 * Old demo code that needs to be ported
 * to the proper 
 * DefaultPipelineBuilderInternal implementation.
 */
use super::DefaultPipelineBuilder;

use std::mem;
use std::rc::Rc;

use gfx_hal as hal;
use gfx_hal::{Device, DescriptorPool};
use gfx_hal::{pso, pass, image as i, format as f};
use failure::Error;

/**
 * Defines internal operations of a DefaultPipelineBuilder.
 */
pub trait DefaultPipelineBuilderInternal<B: hal::Backend> {
	fn create_descriptor_set_layout(&self, device: Rc<&B::Device>) -> B::DescriptorSetLayout;

	fn create_pipeline_layout(&self, device: Rc<&B::Device>) -> B::PipelineLayout;


	//Specify the single render pass'
	//layout.
	fn create_render_pass(&self, device: device: Rc<&B::Device>) -> B::RenderPass;

	fn create_pipeline(&self, device: device: Rc<&B::Device>) -> Result<B::GraphicsPipeline, Error>;

	//Create external-facing descriptors
		//the render calls can bind to.
	fn create_descriptor_pool(&self, device: device: Rc<&B::Device>) -> B::DescriptorPool;

	//Builds the entire pipeline.
	fn build_pipeline(&self, device: Rc<&B::Device>, surface_format: f::Format) -> Result<RenderPipeline<B>, Error>;
}

impl<B: hal::Backend> DefaultPipelineBuilderInternal<B> for DefaultPipelineBuilder<B> {
	fn create_descriptor_set_layout(&self, device: Rc<&B::Device>) -> B::DescriptorSetLayout {
		//Describe pipeline inputs:
		//	First up are the uniforms,
		//	the texture and texture sampler.
		//	Both will be used by the frag shader
		device.create_descriptor_set_layout(&[
				pso::DescriptorSetLayoutBinding {
					binding: 0,
					ty: pso::DescriptorType::SampledImage,
					count: 1,
					stage_flags: pso::ShaderStageFlags::FRAGMENT,
				},
				pso::DescriptorSetLayoutBinding {
					binding: 1,
					ty: pso::DescriptorType::Sampler,
					count: 1,
					stage_flags: pso::ShaderStageFlags::FRAGMENT,
				},
			],
		)
	}

	fn create_pipeline_layout(&self, device: Rc<&B::Device>) -> B::PipelineLayout {
		device.create_pipeline_layout(
			Some(&set_layout),
			&[
				(pso::ShaderStageFlags::VERTEX, 0..8),
				//(pso::ShaderStageFlags::FRAGMENT, 0..1)
			],
		)
	}

	fn create_render_pass(&self, device: device: Rc<&B::Device>) -> B::RenderPass {
		//Describe the destination surface's
		//attributes
		let attachment = pass::Attachment {
			format: Some(surface_format),
			ops: pass::AttachmentOps::new(pass::AttachmentLoadOp::Clear, pass::AttachmentStoreOp::Store),
			stencil_ops: pass::AttachmentOps::DONT_CARE,
			layouts: i::ImageLayout::Undefined .. i::ImageLayout::Present,
		};

		//Indicate to subpass that index
		//0 has a color output attachment
		//(the framebuffer).
		let subpass = pass::SubpassDesc {
			colors: &[(0, i::ImageLayout::ColorAttachmentOptimal)],
			depth_stencil: None,
			inputs: &[],
			preserves: &[],
		};

		//Link the subpass to this render pass
		let dependency = pass::SubpassDependency {
			passes: pass::SubpassRef::External .. pass::SubpassRef::Pass(0),
			stages: pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT .. pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
			accesses: i::Access::empty() .. (i::Access::COLOR_ATTACHMENT_READ | i::Access::COLOR_ATTACHMENT_WRITE),
		};

		//And actually create the render pass here.
		device.create_render_pass(&[attachment], &[subpass], &[dependency])
	}

	fn create_pipeline(&self, device: device: Rc<&B::Device>) -> Result<B::GraphicsPipeline, Error> {
		//Load the actual shader files here.
		unimplemented!();
		// let vs_module = device
		// 	.create_shader_module(include_bytes!("data/vert.spv"))
		// 	.unwrap();
		// let fs_module = device
		// 	.create_shader_module(include_bytes!("data/frag.spv"))
		// 	.unwrap();
		let vs_module = device
			.create_shader_module(&[0u8])?;
		let fs_module = device
			.create_shader_module(&[0u8])?;
		unimplemented!();
		const ENTRY_NAME: &str = "TODO";

		let pipeline = {
			//Specify the entry points
			//for the shader types:
			let (vs_entry, fs_entry) = (
				//The vertex shader
				pso::EntryPoint::<B> {
					entry: ENTRY_NAME,
					module: &vs_module,
					specialization: &[
						pso::Specialization {
							//This will be constant 0.
							id: 0,
							//...Presumably
							//this is used as the z position
							//for the vertices?
							value: pso::Constant::F32(0.8),
						}
					],
				},
				//and the fragment shader.
				pso::EntryPoint::<B> {
					entry: ENTRY_NAME,
					module: &fs_module,
					specialization: &[],
				},
			);

			//Specify all the entry points used by this
			//render pass.
			//We only have one VS and one FS, so
			//just point everything to them.
			let shader_entries = pso::GraphicsShaderSet {
				vertex: vs_entry,
				hull: None,
				domain: None,
				geometry: None,
				fragment: Some(fs_entry),
			};

			//Specify the pipeline's only
			//subpass.
			//Note that it's part of the only render pass.
			let subpass = pass::Subpass { index: 0, main_pass: &render_pass };

			//Create the pipeline description!
			let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
				//We're using these shader entry points
				shader_entries,
				//The shaders take triangle lists
				//as submitted primitives
				hal::Primitive::TriangleList,
				//They'll rasterize primitives as filled fragments
				pso::Rasterizer::FILL,
				//And the actual layout of the pipeline
				//is here
				&pipeline_layout,
				//This pipeline belongs to this
				//subpass
				subpass,
			);
			//That gives the basic behavior of
			//the pipeline, but now we need
			//to provide all the linking details.
			//
			//How do we blend new values?
			pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
				pso::ColorMask::ALL,
				pso::BlendState::ALPHA,
			));
			//What's the buffer layout for
			//vertex data?
			pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
				stride: mem::size_of::<Vertex>() as u32,
				rate: 0,
			});

			//What attributes are available?
			//There's just two: the position vertex
			pipeline_desc.attributes.push(pso::AttributeDesc {
				location: 0,
				binding: 0,
				element: pso::Element {
					//Which uses RG32 = R32, G32
					//per vertex.
					//The demo doesn't use RGBA8
					//or RGB10A2
					//since what's being rendered
					//is effectively a 2D object;
					//The z and w coordinates can be a
					//shader constant instead
					format: f::Format::Rg32Float,
					offset: 0,
				},
			});
			//and the UV vertex
			pipeline_desc.attributes.push(pso::AttributeDesc {
				location: 1,
				binding: 0,
				element: pso::Element {
					//Which uses F32s that come
					//after the position data
					//for U and V
					format: f::Format::Rg32Float,
					offset: 8
				},
			});

			//Generate the final pipeline state object (PSO)
			//here.
			device.create_graphics_pipeline(&pipeline_desc)
		};

		//The PSO is built, so we don't need the
		//shader modules anymore
		device.destroy_shader_module(vs_module);
		device.destroy_shader_module(fs_module);

		Ok(pipeline)
	}

	fn create_descriptor_pool(&self, device: device: Rc<&B::Device>) -> B::DescriptorPool {
		// Descriptor pool -
		// this describes how many descriptors
		// can be allocated at any given time
		// and in how many sets of the given layout.
		device.create_descriptor_pool(
			1, // sets
			&[
				pso::DescriptorRangeDesc {
					ty: pso::DescriptorType::SampledImage,
					count: 1,
				},
				pso::DescriptorRangeDesc {
					ty: pso::DescriptorType::Sampler,
					count: 1,
				},
			],
		)
	}

	fn build_pipeline(&self, device: Rc<&B::Device>, surface_format: f::Format) -> Result<RenderPipeline<B>, Error> {
		let set_layout = self.create_descriptor_set_layout(device);

		let pipeline_layout = self.create_pipeline_layout(device);

		let render_pass = self.create_render_pass(device);

		let pipeline = self.create_pipeline(device)?;

		let mut desc_pool = self.create_descriptor_pool(device);
		let desc_set = desc_pool.allocate_set(&set_layout);

		Ok(pipeline)
	}
}