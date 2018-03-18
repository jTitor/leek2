/*!
 * Defines the ShaderEntryPoint struct.
 */
use std::marker::PhantomData;

use gfx_hal::{self as hal, pso};

/**
 * Describes what function acts as a
 * given shader module's entry point,
 * and the values of all constants
 * used by that shader module.
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct ShaderEntryPoint<'a, B: hal::Backend> {
	//TODO: make all of these templates
	//where T: Into<&whatever>

	/** Name of the shader's entry point function. */
	pub entry: &'a str,
	/**
	 * The shader module's code as a byte slice.
	 * The code is stored as this instead of a
	 * B::ShaderModule to avoid needing a
	 * B::Device::create_shader_module() call
	 * until the actual instantiation of the shader.
	 */
	pub module: &'a [u8],
	/**
	 * An array of specialization information.
	 * This is mostly used for specifying
	 * constant values for the shader.
	 */
	pub specialization: &'a [pso::Specialization],

	
	_backend_type: PhantomData<B>
}

impl<'a, B: hal::Backend> ShaderEntryPoint<'a, B> {
	pub fn new(entry: &'a str, module: &'a [u8], specialization: &'a [pso::Specialization]) -> Self {
		Self {
			entry: entry,
			module: module,
			specialization: specialization,
			_backend_type: PhantomData
		}
	}
}
