/*!
 * Defines the PipelineBlendDescription struct.
 */
use gfx_hal::pso;

/**
 * Describes how a pipeline should blend its result
 * fragments with any existing fragment values
 * in the output targets.
 */
pub type PipelineBlendDescription = pso::ColorBlendDesc;