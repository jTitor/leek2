/*!
 * Contains modules defining the default pipeline builder.
 */

mod default_pipeline_builder;
pub use self::default_pipeline_builder::DefaultPipelineBuilder;

mod internal;
use self::internal::DefaultPipelineBuilderInternal;