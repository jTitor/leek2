/*!
 * Organizes the rendering process into
 * specific passes, subpasses, inputs
 * and outputs.
 * 
 * #Pipeline Layout
 * The pipeline is effectively a big
 * graph, with input nodes specified
 * by *descriptor sets* and *buffer sets*, processing nodes
 * by *passes*, and outputs by *render targets*.
 * Passes can then have subpasses for distinct render
 * steps
 * 
 * At bare minimum, a pipeline will have one vertex buffer
 * feeding one pass containing one subpass, which
 * then outputs to a framebuffer render target.
 * 
 * ## Buffers vs. Descriptors
 * It's not clear why each is preferred, but it looks like
 * buffers are used for per-vertex data and descriptors
 * are used for everything else, like images and
 * texture samplers.
 */

mod render_pipeline;
pub use self::render_pipeline::RenderPipeline;

mod default_pipeline;
pub use self::default_pipeline::{DefaultPipeline, DefaultPipelineBuilder};

mod image;

mod render_target;

mod sampler;

mod descriptor_set;