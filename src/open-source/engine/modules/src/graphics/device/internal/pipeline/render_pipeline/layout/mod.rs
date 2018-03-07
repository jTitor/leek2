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

mod graphics_pipeline_description;
pub use self::graphics_pipeline_description::GraphicsPipelineDescription;

mod graphics_shader_set;
pub use self::graphics_shader_set::GraphicsShaderSet;

mod shader_entry_point;
pub use self::shader_entry_point::ShaderEntryPoint;

mod pipeline_blend_description;
pub use self::pipeline_blend_description::PipelineBlendDescription;

mod attribute_description;
pub use self::attribute_description::AttributeDescription;

mod buffer_description;
pub use self::buffer_description::BufferDescription;

mod buffer_element;
pub use self::buffer_element::BufferElement;