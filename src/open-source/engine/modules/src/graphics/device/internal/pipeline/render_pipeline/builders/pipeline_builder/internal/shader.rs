/*!
 * Defines internal methods of PipelineBuilder
 * related to shader loading and unloading.
 */
use super::SubpassPipelineBuildError;
use super::super::{SubpassPipelineLayout, SubpassPipelineLayoutRequiredInfo};
use graphics::device::internal::pipeline::PipelineBuilder;
use graphics::device::internal::pipeline::render_pipeline::layout;

use std::collections::HashMap;

use failure::Error;
use gfx_hal::{self as hal, pso,
Backend, Device};
use gfx_hal::pso::{GraphicsShaderSet, EntryPoint};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
type UnloadedShaderHashKey = ShaderEntryType;
type UnloadedShaderHashValue<'a, B: hal::Backend> = Option<layout::ShaderEntryPoint<'a, B>>;
type UnloadedShaderHashMap<'a, B: hal::Backend> = HashMap<UnloadedShaderHashKey, UnloadedShaderHashValue<'a, B>>;
type LoadedShaderHashKey = ShaderEntryType;
type LoadedShaderHashValue<'a, B: hal::Backend> = Option<pso::EntryPoint<'a, B>>;
type LoadedShaderHashMap<'a, B: hal::Backend> = HashMap<LoadedShaderHashKey, LoadedShaderHashValue<'a, B>>;
pub trait ShaderLoad<B: hal::Backend> {
	fn load_shader_entry_point(&self, entry_point: UnloadedShaderHashValue<B>) -> Result<LoadedShaderHashValue<B>, Error>;

	fn init_shader_map(&self, device: &B::Device, subpass_layout: SubpassPipelineLayout<B>) -> Result<LoadedShaderHashMap<B>, Error>;

	fn unload_shader_map(&self, device: &B::Device, shader_map: &LoadedShaderHashMap<B>);

	fn shader_map_to_shader_set(&self, shader_map: &LoadedShaderHashMap<B>) -> Result<pso::GraphicsShaderSet<B>, Error>;
}

impl<'a, B: hal::Backend> ShaderLoad<B> for PipelineBuilder<'a, B> {
	fn load_shader_entry_point(&self, entry_point: UnloadedShaderHashValue<B>) -> Result<LoadedShaderHashValue<B>, Error> {
		match entry_point {
			None => {
				//If this is none, just return none.
				return Ok(None);
			},
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
				let result = pso::EntryPoint::<B> {
					entry: raw_entry_point.entry,
					module: &loaded_module,
					specialization: raw_entry_point.specialization,
				};

				//and return.
				return Ok(Some(result));
			}
		}

		unreachable!();
	}

	fn init_shader_map(&self, device: &B::Device, subpass_layout: SubpassPipelineLayout<B>) -> Result<LoadedShaderHashMap<B>, Error> {
		//Try to load all of the shaders.
		let shaders = [(Some(subpass_layout.required_info.vertex_shader_entry), ShaderEntryType::Vertex),
		(subpass_layout.fragment_shader_entry, ShaderEntryType::Fragment),
		(subpass_layout.hull_shader_entry, ShaderEntryType::Hull),
		(subpass_layout.domain_shader_entry, ShaderEntryType::Domain),
		(subpass_layout.geometry_shader_entry, ShaderEntryType::Geometry)];
		let mut result_map: HashMap<ShaderEntryType, pso::EntryPoint<B>> = HashMap::new();

		//Perform a error-mapped load.
		//Only fully-loaded entry points should
		//be in the result map.
		let result = {
			for &(shader_option, shader_enum) in &shaders {
				let loaded_shader = self.load_shader_entry_point(shader_option.clone())?;

				result_map.insert(*shader_enum, loaded_shader);
			}

			Ok(result_map)
		};

		//If we failed, unload all
		//the elements in the result map.
		if let Err(_) = result {
			self.unload_shader_map(result_map);
		}

		//Now return our result.
		result
	}

	fn unload_shader_map(&self, device: &B::Device, shader_map: &LoadedShaderHashMap<B>) {
		for (key, val) in shader_map {
			device.destroy_shader_module(val.module);
		}
	}

	fn shader_map_to_shader_set(&self, shader_map: &LoadedShaderHashMap<B>) -> Result<pso::GraphicsShaderSet<B>, Error> {
		//Vertex entry point must exist...
		if let Some(vertex_entry) = unimplemented!() {
			//...but otherwise everything
			//else is a direct mapping

			return Ok(pso::GraphicsShaderSet::<B> {
				vertex: vertex_entry,
				hull: shader_map.get(&ShaderEntryType::Hull).cloned(),
				domain: shader_map.get(&ShaderEntryType::Domain).cloned(),
				geometry: shader_map.get(&ShaderEntryType::Geometry).cloned(),
				fragment: shader_map.get(&ShaderEntryType::Fragment).cloned()
			});
		}
		else {
			debug_assert!(false, "Vertex entry point missing from shader list");

			return Err(SubpassPipelineBuildError::VertexShaderEntryPointNotFound);
		}

		unreachable!();
	}
}