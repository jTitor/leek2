/*!
 * Default pipeline implementation.
 * This generates a pipeline with one vertex shader
 * and one fragment shader.
 */
use super::RenderPipeline;

use failure::Error;

struct DefaultPipeline {}
impl RenderPipeline for DefaultPipeline {}

struct DefaultPipelineBuilder {}
impl DefaultPipelineBuilder {
	fn build(device: &mut Device) -> Result<DefaultPipeline, Error> {
		//Describe pipeline inputs:
		//	First up are the uniforms,
		//	the texture and texture sampler.
		//	Both will be used by the frag shader
		let set_layout = device.create_descriptor_set_layout(&[
					pso::DescriptorSetLayoutBinding {
						binding: 0,
						ty: pso::DescriptorType::SampledImage,
						count: 1,
						stage_flags: ShaderStageFlags::FRAGMENT,
					},
					pso::DescriptorSetLayoutBinding {
						binding: 1,
						ty: pso::DescriptorType::Sampler,
						count: 1,
						stage_flags: ShaderStageFlags::FRAGMENT,
					},
				],
			);

		//Next, the varying data in a push block.
		//We only need a couple of vertex
		//attributes 
		let pipeline_layout = device.create_pipeline_layout(
			Some(&set_layout),
			&[
				(pso::ShaderStageFlags::VERTEX, 0..8),
			],
		);

		//Specify the single render pass'
		//layout.
		let render_pass = {
			//Describe the destination surface's
			//attributes
			let attachment = pass::Attachment {
				format: Some(surface_format),
				ops: pass::AttachmentOps::new(pass::AttachmentLoadOp::Clear, pass::AttachmentStoreOp::Store),
				stencil_ops: pass::AttachmentOps::DONT_CARE,
				layouts: i::ImageLayout::Undefined .. i::ImageLayout::Present,
			};

			//We don't really have a subpass,
			//just send color results through
			let subpass = pass::SubpassDesc {
				colors: &[(0, i::ImageLayout::ColorAttachmentOptimal)],
				depth_stencil: None,
				inputs: &[],
				preserves: &[],
			};

			//Link the subpass to this render pass
			let dependency = pass::SubpassDependency {
				passes: pass::SubpassRef::External .. pass::SubpassRef::Pass(0),
				stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT .. PipelineStage::COLOR_ATTACHMENT_OUTPUT,
				accesses: i::Access::empty() .. (i::Access::COLOR_ATTACHMENT_READ | i::Access::COLOR_ATTACHMENT_WRITE),
			};

			//And actually create the render pass here.
			device.create_render_pass(&[attachment], &[subpass], &[dependency])
		};

		let pipeline = {
			//Load the actual shader files here.
			let vs_module = device
				.create_shader_module(include_bytes!("data/vert.spv"))
				.unwrap();
			let fs_module = device
				.create_shader_module(include_bytes!("data/frag.spv"))
				.unwrap();

			let pipeline = {
				//Specify the entry points
				//for the shader types:
				let (vs_entry, fs_entry) = (
					//The vertex shader
					pso::EntryPoint::<back::Backend> {
						entry: ENTRY_NAME,
						module: &vs_module,
						specialization: &[
							Specialization {
								id: 0,
								//...Presumably
								//this is used as the z position
								//for the vertices?
								value: pso::Constant::F32(0.8),
							}
						],
					},
					//and the fragment shader.
					pso::EntryPoint::<back::Backend> {
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
				let subpass = Subpass { index: 0, main_pass: &render_pass };

				//Create the pipeline description!
				let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
					//We're using these shader entry points
					shader_entries,
					//The shaders take triangle lists
					//as submitted primitives
					Primitive::TriangleList,
					//They'll rasterize primitives as filled fragments
					pso::Rasterizer::FILL,
					//And the actual layout of the pipeline
					//is here
					&pipeline_layout,
					//using this subpass
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
					stride: std::mem::size_of::<Vertex>() as u32,
					rate: 0,
				});

				//What attributes are available?
				//There's just two: the position vertex
				pipeline_desc.attributes.push(pso::AttributeDesc {
					location: 0,
					binding: 0,
					element: pso::Element {
						//Which uses F32s
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

			pipeline
		};

		//Create external-facing descriptors
		//the render calls can bind to.
		// Descriptors
		let mut desc_pool = device.create_descriptor_pool(
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
		);
		let desc_set = desc_pool.allocate_set(&set_layout);
		//TODO_rust: make these part of result struct

		pipeline
	}
}