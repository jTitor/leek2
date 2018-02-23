/*!
 * Performs the actual rendering for a Device.
 * A DeviceController uses this pipeline to decide what
 * draw calls must be executed.
 */
use gfx_hal as hal;

use failure::Error;

pub struct RenderPipeline {}

pub struct RenderPipelineBuilder{}

impl RenderPipelineBuilder {
	pub fn build() -> Result<RenderPipeline, Error> {
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

		let pipeline_layout = device.create_pipeline_layout(
			Some(&set_layout),
			&[
				(pso::ShaderStageFlags::VERTEX, 0..8),
			],
		);

		let render_pass = {
			let attachment = pass::Attachment {
				format: Some(surface_format),
				ops: pass::AttachmentOps::new(pass::AttachmentLoadOp::Clear, pass::AttachmentStoreOp::Store),
				stencil_ops: pass::AttachmentOps::DONT_CARE,
				layouts: i::ImageLayout::Undefined .. i::ImageLayout::Present,
			};

			let subpass = pass::SubpassDesc {
				colors: &[(0, i::ImageLayout::ColorAttachmentOptimal)],
				depth_stencil: None,
				inputs: &[],
				preserves: &[],
			};

			let dependency = pass::SubpassDependency {
				passes: pass::SubpassRef::External .. pass::SubpassRef::Pass(0),
				stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT .. PipelineStage::COLOR_ATTACHMENT_OUTPUT,
				accesses: i::Access::empty() .. (i::Access::COLOR_ATTACHMENT_READ | i::Access::COLOR_ATTACHMENT_WRITE),
			};

			device.create_render_pass(&[attachment], &[subpass], &[dependency])
		};

		//
		let pipeline = {
			let vs_module = device
				.create_shader_module(include_bytes!("data/vert.spv"))
				.unwrap();
			let fs_module = device
				.create_shader_module(include_bytes!("data/frag.spv"))
				.unwrap();

			/*
			#[cfg(all(feature = "metal", feature = "metal_argument_buffer"))]
			let shader_lib = device.create_shader_library_from_source(
					include_str!("shader/quad_indirect.metal"),
					back::LanguageVersion::new(2, 0),
				).expect("Error on creating shader lib");
			*/

			let pipeline = {
				let (vs_entry, fs_entry) = (
					pso::EntryPoint::<back::Backend> {
						entry: ENTRY_NAME,
						module: &vs_module,
						specialization: &[
							Specialization {
								id: 0,
								value: pso::Constant::F32(0.8),
							}
						],
					},
					pso::EntryPoint::<back::Backend> {
						entry: ENTRY_NAME,
						module: &fs_module,
						specialization: &[],
					},
				);

				/*
				#[cfg(all(feature = "metal", feature = "metal_argument_buffer"))]
				let (vs_entry, fs_entry) = (
					pso::EntryPoint {
						entry: "vs_main",
						module: &shader_lib,
						specialization: &[
							Specialization {
								id: 0,
								value: pso::Constant::F32(0.8),
							}
						],
					},
					pso::EntryPoint {
						entry: "ps_main",
						module: &shader_lib,
						specialization: &[],
					},
				);
				*/

				let shader_entries = pso::GraphicsShaderSet {
					vertex: vs_entry,
					hull: None,
					domain: None,
					geometry: None,
					fragment: Some(fs_entry),
				};

				let subpass = Subpass { index: 0, main_pass: &render_pass };

				let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
					shader_entries,
					Primitive::TriangleList,
					pso::Rasterizer::FILL,
					&pipeline_layout,
					subpass,
				);
				pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
					pso::ColorMask::ALL,
					pso::BlendState::ALPHA,
				));
				pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
					stride: std::mem::size_of::<Vertex>() as u32,
					rate: 0,
				});

				pipeline_desc.attributes.push(pso::AttributeDesc {
					location: 0,
					binding: 0,
					element: pso::Element {
						format: f::Format::Rg32Float,
						offset: 0,
					},
				});
				pipeline_desc.attributes.push(pso::AttributeDesc {
					location: 1,
					binding: 0,
					element: pso::Element {
						format: f::Format::Rg32Float,
						offset: 8
					},
				});


				device.create_graphics_pipeline(&pipeline_desc)
			};

			device.destroy_shader_module(vs_module);
			device.destroy_shader_module(fs_module);
			/*
			#[cfg(all(feature = "metal", feature = "metal_argument_buffer"))]
			device.destroy_shader_module(shader_lib);
			*/

			pipeline
		};

		Ok(pipeline)
	}
}