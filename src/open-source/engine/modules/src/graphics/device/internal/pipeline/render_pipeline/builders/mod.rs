/*!
 * Defines builder structs for a RenderPipeline.
 */
mod default_pipeline_builder;
pub use self::default_pipeline_builder::DefaultPipelineBuilder;

mod pipeline_builder;
pub use self::pipeline_builder::PipelineBuilder;

mod destroy_on_drop;
use self::destroy_on_drop::{DestroyOnDrop, DestroyIterOnDrop};