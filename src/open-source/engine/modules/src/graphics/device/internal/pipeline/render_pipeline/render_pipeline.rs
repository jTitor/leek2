/*!
 * Performs the actual rendering for a Device.
 * A DeviceController uses this pipeline to decide what
 * draw calls must be executed.
 */
use failure::Error;

pub trait RenderPipeline {
	//The exact backend doesn't matter,
	//so much as how data is submitted to it.
}