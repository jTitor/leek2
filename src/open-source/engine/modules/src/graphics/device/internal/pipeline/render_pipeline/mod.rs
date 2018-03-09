/*!
 * Organizes the rendering process into
 * specific passes, subpasses, inputs
 * and outputs.
 * 
 * #Rendering System Layout
 * The rendering system is effectively a big
 * graph, broken up into stages called *passes*.
 * Each pass is then broken up into at least one *subpass*
 * that performs that pass' rendering/calculations, with
 * the subpasses' inputs described by *descriptor sets*,
 * *buffer sets* and *shader sets*, while their
 * outputs are described by *render targets*.
 * To perform rendering, you then submit a render
 * *submission* that specifies a pass
 * to render, gives the pass the input data its
 * subpasses need, and points the pass to the outputs
 * its subpasses need.
 * The backend will then run the subpasses in the most
 * optimal order it can for the submission.
 * 
 * At bare minimum, a rendering system will have one vertex buffer
 * feeding one pass containing one subpass, which
 * then outputs to a framebuffer render target.
 * 
 * ## Buffers vs. Descriptors
 * It's not clear why each is preferred, but it looks like
 * buffers are used for per-vertex data and descriptors
 * are used for everything else, like images and
 * texture samplers.
 */

mod pipeline;
pub use self::pipeline::Pipeline;

mod builders;
pub use self::builders::{DefaultPipelineBuilder, PipelineBuilder};

pub mod elements;

pub mod layout;