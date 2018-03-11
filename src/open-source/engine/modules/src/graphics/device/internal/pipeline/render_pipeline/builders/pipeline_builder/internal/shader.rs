/*!
 * Defines internal methods of PipelineBuilder
 * related to shader loading and unloading.
 */
use graphics::device::internal::pipeline::PipelineBuilder;
use super::super::{SubpassPipelineLayout, SubpassPipelineLayoutRequiredInfo};
use graphics::device::internal::pipeline::render_pipeline::layout;

use std::collections::HashMap;

use failure::Error;
use gfx_hal::{self as hal, pso,
Backend, Device, ShaderEntryPoint};
use gfx_hal::pso::GraphicsShaderSet;

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

	fn init_shader_map(&self, device: &B::Device, subpass_layout: SubpassPipelineLayout<B>) -> Result<HashMap<ShaderEntryType, B::ShaderEntryPoint>, Error>;

	fn unload_shader_map(&self, device: &B::Device, shader_map: &HashMap<ShaderEntryType, B::ShaderEntryPoint>);

	fn shader_map_to_shader_set(&self shader_map: &HashMap<ShaderEntryType, B::ShaderEntryPoint>) -> pso::GraphicsShaderSet<B>;
}

impl<B: hal::Backend> ShaderLoad<B> for PipelineBuilder<B> {
	fn load_shader_entry_point(&self, entry_point: Option<layout::ShaderEntryPoint<B>>) -> Result<Option<B::ShaderEntryPoint>, Error> {
		match entry_point {
			//If this is none, just return none.
			None => { return Ok(None); },
			Some(raw_entry_point) => {
				//Otherwise, load the bytes,
				// let vs_module = device
				// 	.create_shader_module(include_bytes!("data/vert.spv"))
				// 	.unwrap();
				// let fs_module = device
				// 	.create_shader_module(include_bytes!("data/frag.spv"))
				// 	.unwrap();
				let loaded_module = unimplemented!();
				
				//pack that into
				//a backend SEP,
				let result = pso::EntryPoint::<back::Backend> {
					entry: raw_entry_point.entry,
					module: &loaded_module,
					specialization: raw_entry_point.specialization,
				};

				//and return.
				return Ok(result);
			}
		}

		unreachable!();
	}

	fn init_shader_map(&self, device: &B::Device, subpass_layout: SubpassPipelineLayout<B>) -> Result<HashMap<ShaderEntryType, B::ShaderEntryPoint>, Error> {
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
				self.unload_shader_map(result_map);
			}
		}

		//Now return our result.
		result
	}

	fn unload_shader_map(&self, device: &B::Device, shader_map: &HashMap<ShaderEntryType, B::ShaderEntryPoint>) {
		for (key, val) in shader_map {
			device.destroy_shader_module(val.module);
		}
	}

	fn shader_map_to_shader_set(&self shader_map: &HashMap<ShaderEntryType, B::ShaderEntryPoint>) -> Result<pso::GraphicsShaderSet<B>, Error> {
		//Vertex entry point must exist...
		if let Some(vertex_entry) = unimplemented!() {
			//...but otherwise everything
			//else is a direct mapping

			return Ok(pso::GraphicsShaderSet::<B> {
				vertex: vertex_entry,
				hull: shader_map.get(ShaderEntryType::Hull),
				domain: shader_map.get(ShaderEntryType::Domain),
				geometry: shader_map.get(ShaderEntryType::Geometry),
				fragment: shader_map.get(ShaderEntryType::Fragment)
			});
		}
		else {
			assert_debug!(false, "Vertex entry point missing from shader list");

			return Error;
		}

		unreachable!();
	}
}