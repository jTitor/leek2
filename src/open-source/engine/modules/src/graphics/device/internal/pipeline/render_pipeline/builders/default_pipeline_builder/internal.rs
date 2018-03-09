/*!
 * Defines the DefaultPipelineBuilderInternal trait.
 */
use super::DefaultPipelineBuilder;
use super::super::{RenderPassLayout, SubpassPipelineLayout, SubpassPipelineLayoutRequiredInfo};
use graphics::device::internal::pipeline::{layout, elements};
use layout::ShaderEntryPoint;

use std::mem;
use std::rc::Rc;

use gfx_hal::{Device, DescriptorPool, self as hal, pso, pass, format as f, image as i};
use failure::Error;

/**
 * Defines internal operations of a DefaultPipelineBuilder.
 */
pub trait DefaultPipelineBuilderInternal<B: hal::Backend> {
	fn create_render_pass_layouts(&self, device: Rc<&B::Device>) -> Result<Vec<RenderPassLayout>, Error>;

	fn create_subpass_pipeline_layouts(&self, device: Rc<&B::Device>) -> Result<Vec<SubpassPipelineLayout<B>>, Error>;

	fn configure_render_pass_layout(&self, in_rp_layout: &mut RenderPassLayout);

	fn create_subpass_required_info(&self, entry_point_name: &str, vs_module_bytes: &[u8]) -> SubpassPipelineLayoutRequiredInfo<B>;

	fn configure_subpass_pipeline_layout(&self, required_info: SubpassPipelineLayoutRequiredInfo<B>, entry_point_name: &str, fs_module_bytes: &[u8]) -> SubpassPipelineLayout<B>
}

impl<B: hal::Backend> DefaultPipelineBuilderInternal for DefaultPipelineBuilder<B> {
	fn create_render_pass_layouts(&self, device: Rc<&B::Device>, surface_format: f::Format) -> Result<Vec<RenderPassLayout>, Error> {
		let mut result = Vec::<RenderPassLayout>::new();

		//This just has one renderpass.
		let mut render_pass = RenderPassLayout::new();
		//TODO: adapter provides this,
		//figure out where to put this
		let surface_format = ;
		
		render_pass.attachments.push(elements::Attachment {
			format: Some(surface_format),
			ops: pass::AttachmentOps::new(pass::AttachmentLoadOp::Clear, pass::AttachmentStoreOp::Store),
			stencil_ops: pass::AttachmentOps::DONT_CARE,
			layouts: i::ImageLayout::Undefined .. i::ImageLayout::Present,
		});

		render_pass.subpass_descriptions.push(layout::SubpassDescription {
			colors: &[(0, i::ImageLayout::ColorAttachmentOptimal)],
			depth_stencil: None,
			inputs: &[],
			preserves: &[],
		});

		render_pass.dependencies.push(elements::SubpassDependency {
			passes: pass::SubpassRef::External .. pass::SubpassRef::Pass(0),
			stages: pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT .. pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
			accesses: i::Access::empty() .. (i::Access::COLOR_ATTACHMENT_READ | i::Access::COLOR_ATTACHMENT_WRITE),
		});
		unimplemented!();

		result.push(render_pass);

		Ok(result)
	}

	fn create_subpass_pipeline_layouts(&self, device: Rc<&B::Device>) -> Result<Vec<SubpassPipelineLayout<B>>, Error> {
		let mut result = Vec::<SubpassPipelineLayout<B>>::new();

		//Similarly, this has one vertex shader
		//and one fragment shader.

		//Generate those shader's entries...
		const ENTRY_POINT_NAME: &str = "TODO";
		let vs_module_bytes = &[0u8];
		let fs_module_bytes = &[0u8];

		let mut subpass_required = self.create_subpass_required_info(ENTRY_POINT_NAME,
		vs_module_bytes,
		fs_module_bytes);

		//Now make the actual subpass pipeline layout.
		let mut subpass = self.configure_subpass_pipeline_layout(subpass_required);

		//Layout setup complete,
		//push it into the result vec.
		result.push(subpass);

		Ok(result)
	}

	fn configure_render_pass_layout(&self, in_rp_layout: &mut RenderPassLayout) {
		unimplemented!();
	}

	fn create_subpass_required_info(&self, entry_point_name: &str, vs_module_bytes: &[u8]) -> SubpassPipelineLayoutRequiredInfo<B> {
		unimplemented!();
		let vertex_shader = ShaderEntryPoint::<B> {
			entry: entry_point_name,
			module: vs_module_bytes,
			specialization: &[
				pso::Specialization {
					//Z coordinate of vertices
					//at constant 0.
					id: 0,
					value: pso::Constant::F32(0.8),
				}
			]
		};

		//...and put them in the subpass layout.
		SubpassPipelineLayoutRequiredInfo::<B> {
			vertex_shader_entry: vertex_shader,
			render_pass_index: 0,
			subpass_index: 0
		}
	}

	fn configure_subpass_pipeline_layout(&self, required_info: SubpassPipelineLayoutRequiredInfo<B>, entry_point_name: &str, fs_module_bytes: &[u8]) -> SubpassPipelineLayout<B> {
		let mut subpass = SubpassPipelineLayout::<B>::new(required_info);

		let frag_shader = ShaderEntryPoint::<B> {
			entry: entry_point_name,
			module: fs_module_bytes,
			specialization: &[]
		};

		subpass.fragment_shader_entry = Some(frag_shader);
		unimplemented!();
		
		subpass.set_layout_bindings = unimplemented!();
		subpass.push_block_constants = unimplemented!();
		
		//That gives the basic behavior of
		//the pipeline, but now we need
		//to provide all the linking details.
		
		//How do we blend new values?
		subpass.blending_target_descriptions.push(
			layout::PipelineBlendDescription {
				pso::ColorMask::ALL,
				pso::BlendState::ALPHA,
			}
		);
		//What's the buffer layout for
		//vertex data?
		subpass.vertex_buffer_descriptions.push(
			layout::VertexBufferDescription {
				stride: mem::size_of::<Vertex>() as u32,
				rate: 0,
			}
		);
		//What attributes are available?
		//There's just two: the position vertex...
		subpass.attribute_descriptions.push(
			layout::AttributeDescription {
				location: 0,
				binding: 0,
				element: pso::Element {
					//..which uses RG32 = R32, G32
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
			}
		);
		//and the UV vertex...
		subpass.attribute_descriptions.push(
			layout::AttributeDescription {
				location: 1,
				binding: 0,
				element: pso::Element {
					//...which uses F32s that come
					//after the position data
					//for U and V
					format: f::Format::Rg32Float,
					offset: 8
				},
			}
		);

		//The shaders take triangle lists
		//as submitted primitives
		subpass.primitive_type = hal::Primitive::TriangleList;
		//and rasterize primitives as filled fragments.
		subpass.rasterization_type = pso::Rasterizer::FILL;

		subpass
	}
}