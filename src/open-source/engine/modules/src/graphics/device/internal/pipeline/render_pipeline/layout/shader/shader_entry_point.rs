/*!
 * Defines the ShaderEntryPoint struct.
 */
use gfx_hal::{self as hal, pso};

/**
 * Describes what function acts as a
 * given shader module's entry point,
 * and the values of all constants
 * used by that shader module.
 */
#[derive(Debug, Copy)]
pub struct ShaderEntryPoint<'a, B: hal::Backend> {
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
}