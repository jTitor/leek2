/*!
 * Defines the SubpassPipeline struct.
 */
use graphics::device::internal::pipeline::render_pipeline::{elements, layout};

use std::rc::Rc;

use gfx_hal::{self as hal, pso};

pub struct SubpassPipelineLayoutRequiredInfo<B: hal::Backend> {
	vertex_shader_entry: layout::ShaderEntryPoint<B>,
	//Used to make the Subpass node. 
	render_pass: Rc<&elements::Pass>,
	subpass_index: u32
}

/**
 * Defines the structure of a SubpassPipeline for
 * a PipelineBuilder.
 */
#[derive(Default)]
pub struct SubpassPipelineLayout<B: hal::Backend> {
	pub required_info: SubpassPipelineLayoutRequiredInfo<B>,

	//The shader entry points...
	pub hull_shader_entry: Option<layout::ShaderEntryPoint<B>>,
	pub domain_shader_entry: Option<layout::ShaderEntryPoint<B>>,
	pub geometry_shader_entry: Option<layout::ShaderEntryPoint<B>>,
	pub fragment_shader_entry: Option<layout::ShaderEntryPoint<B>>,
	
	//Used to make the PipelineLayout struct.
	//If None, the set_layout will also be None.
	pub set_layout_bindings: Option<Vec<pso::DescriptorSetLayoutBinding>>,
	pub push_block_constants: Vec<(pso::ShaderStageFlags, Range<u32, u32>)>,

	pub blending_target_descriptions: Vec<pso::ColorBlendDesc>,
	pub vertex_buffer_descriptions: Vec<pso::VertexBufferDesc>,
	pub attribute_descriptions: Vec<pso::AttributeDesc>,

	/**
	 * Describes what kind of primitive
	 * this subpass will accept during submission.
	 */
	pub primitive_type: hal::Primitive,
	/*
	 * Describes how this subpass will rasterize its
	 * rendered triangles.
	 */
	pub rasterization_type: pso::Rasterizer,
	pub pipeline_description: layout::GraphicsPipelineDescription
}

impl<B: hal::Backend> SubpassPipelineLayout<B> {
	pub fn new(required_info: SubpassPipelineLayoutRequiredInfo<B>) -> SubpassPipelineLayout<B> {
		SubpassPipelineLayout::<B> {
			required_info: required_info,
			Default::default()
		}
	}
}