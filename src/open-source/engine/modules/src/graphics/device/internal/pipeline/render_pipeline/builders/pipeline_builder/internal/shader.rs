/*!
 * Defines internal methods of PipelineBuilder
 * related to shader loading and unloading.
 */
use graphics::device::internal::pipeline::PipelineBuilder;
use super::super::{SubpassPipelineLayout, SubpassPipelineLayoutRequiredInfo};
use graphics::device::internal::pipeline::render_pipeline::layout;

use std::collections::HashMap;

use failure::Error;
use gfx_hal::{self as hal, pso, Backend};

enum ShaderEntryType {
	Vertex,
	Fragment,
	Hull,
	Domain,
	Geometry
}

/**
 * Defines internal methods of PipelineBuilder
 * related to shader loading and unloading.
 */
pub trait ShaderLoad<B: hal::Backend> {
	fn load_shader_entry_point(&self, entry_point: Option<layout::ShaderEntryPoint<B>>) -> Result<Option<B::ShaderEntryPoint>, Error>;

	fn init_shader_list(&self, subpass_layout: SubpassPipelineLayout<B>) -> pso::GraphicsShaderSet;
}

impl<B: hal::Backend> ShaderLoad<B> for PipelineBuilder<B> {
	fn load_shader_entry_point(&self, entry_point: Option<layout::ShaderEntryPoint<B>>) -> Result<Option<B::ShaderEntryPoint>, Error> {
		unimplemented!();
		//If this is none, just return none.
		//Otherwise, load the bytes, pack that into
		//a backend SEP, and return.

		Ok()
	}

	fn init_shader_entry_list(&self, subpass_layout: SubpassPipelineLayout<B>) -> Result<HashMap<ShaderEntryType, B::ShaderEntryPoint>, Error> {
		//Try to load all of the shaders.
		let shaders = [(Some(subpass_layout.required_info.vertex_shader_entry), ShaderEntryType::Vertex),
		(subpass_layout.fragment_shader_entry, ShaderEntryType::Fragment),
		(subpass_layout.hull_shader_entry, ShaderEntryType::Hull),
		(subpass_layout.domain_shader_entry, ShaderEntryType::Domain),
		(subpass_layout.geometry_shader_entry, ShaderEntryType::Geometry)];
		let mut result_map = HashMap::<ShaderEntryType, B::ShaderEntryPoint>::new();

		//Perform a error-mapped load.
		//Only fully-loaded entry points should
		//be in the result map.
		let result = {
			for shader_tuple in shaders {
				let (shader_option, shader_enum) = shader_tuple;
				let loaded_shader = self.load_shader_entry_point(shader_option)?;

				result_map.insert(shader_enum, loaded_shader);
			}

			Ok(result_map)
		}

		//If we failed, unload all
		//the elements in the result map.
		match result {
			Ok(_) => {},
			Err(_) => {
				unimplemented!();
			}
		}

		//Now return our result.
		result
	}
}