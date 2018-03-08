/*!
 * Contains modules describing the layout
 * of a rendering pipeline.
 * 
 * These layout structs can't directly be used
 * for rendering, but when combined with
 * shader/texture assets and pipeline nodes/qualifiers
 * they create an actual pipeline that
 * can run rendering submissions.
*/

mod pipeline;
pub use self::pipeline::*;

mod shader;
pub use self::shader::*;

mod buffer;
pub use self::buffer::*;

mod attribute;
pub use self::attribute::*;

mod subpass;
pub use self::subpass::*;