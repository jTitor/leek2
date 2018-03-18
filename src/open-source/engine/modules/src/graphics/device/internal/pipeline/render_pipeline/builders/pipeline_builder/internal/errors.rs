/*!
 * Defines error structs used by PipelineBuilder.
 */
use failure::Error;

#[derive(Debug, Fail)]
pub enum SubpassPipelineBuildError {
	#[fail(display = "Couldn't find a renderpass matching this subpass pipeline layout's renderpass index")]
	RenderPassIndexOutOfRange,
	#[fail(display = "This subpass pipeline layout's shader list needs to have a vertex shader set, but none was found")]
	VertexShaderEntryPointNotFound,
}