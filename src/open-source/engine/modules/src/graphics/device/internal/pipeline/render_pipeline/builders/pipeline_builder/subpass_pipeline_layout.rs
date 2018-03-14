/*!
 * Defines the SubpassPipeline struct.
 */
use graphics::device::internal::pipeline::render_pipeline::{elements, layout};

use std::marker::PhantomData;
use std::ops::Range;

use gfx_hal::{self as hal, pso};

#[derive(Debug, Default)]
pub struct SubpassPipelineLayoutRequiredInfo<'a, B: hal::Backend> {
	pub vertex_shader_entry: layout::ShaderEntryPoint<'a, B>,
	//Used to make the Subpass node.
	
	/**
	 * Index into the builder's render_passes
	 * Vec that points to this subpass' rendering pass.
	 */
	pub render_pass_index: usize,
	/**
	 * TODO
	 */
	pub subpass_index: usize,
	
	_backend_type: PhantomData<B>
}

/**
 * Defines the structure of a SubpassPipeline for
 * a PipelineBuilder.
 */
#[derive(Default)]
pub struct SubpassPipelineLayout<'a, B: hal::Backend> {
	pub required_info: SubpassPipelineLayoutRequiredInfo<'a, B>,

	//The shader entry points...
	pub hull_shader_entry: Option<layout::ShaderEntryPoint<'a, B>>,
	pub domain_shader_entry: Option<layout::ShaderEntryPoint<'a, B>>,
	pub geometry_shader_entry: Option<layout::ShaderEntryPoint<'a, B>>,
	pub fragment_shader_entry: Option<layout::ShaderEntryPoint<'a, B>>,
	
	//Used to make the PipelineLayout struct.
	//If None, the set_layout will also be None.
	pub set_layout_bindings: Option<Vec<pso::DescriptorSetLayoutBinding>>,
	pub push_block_constants: Vec<(pso::ShaderStageFlags, Range<u32>)>,

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
	
	_backend_type: PhantomData<B>
}

impl<'a, B: hal::Backend> SubpassPipelineLayout<'a, B> {
	pub fn new(required_info: SubpassPipelineLayoutRequiredInfo<B>) -> SubpassPipelineLayout<'a, B> {
		SubpassPipelineLayout::<B> {
			required_info: required_info,
			..Default::default()
		}
	}
}