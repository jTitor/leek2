/*!
 * Describes the GraphicsPipelineLayout struct.
 */

/**
 * Fully describes what a pipline will use
 * to perform its graphics rendering operations.
 * 
 * This doesn't describe *how* the pipeline
 * will execute those operations; that is
 * done by a PipelineGraph. 
 */
pub struct GraphicsPipelineDescription {}

//For reference, gfx's GraphicsPipelineLayout:
// pub struct GraphicsPipelineDesc<'a, B: Backend> {
//     /// A list of shaders used by this
//     /// pipeline.
//     pub shaders: GraphicsShaderSet<'a, B>,
//     /// Rasterizer setup
//     pub rasterizer: Rasterizer,
//     /// Vertex buffers (IA)
//     pub vertex_buffers: Vec<VertexBufferDesc>,
//     /// Vertex attributes (IA)
//     pub attributes: Vec<AttributeDesc>,
//     ///
//     pub input_assembler: InputAssemblerDesc,
//     /// How should we blend fragment values together?
//     pub blender: BlendDesc,
//     /// Depth stencil (DSV)
//     pub depth_stencil: Option<DepthStencilDesc>,
//     /// Pipeline layout.
//     pub layout: &'a B::PipelineLayout,
//     /// Subpass in which the pipeline can be executed.
//     pub subpass: pass::Subpass<'a, B>,
//     ///
//     pub flags: PipelineCreationFlags,
//     /// The pipeline this pipeline is a part of.
//     pub parent: BasePipeline<'a, B::GraphicsPipeline>,
// }

// impl<'a, B: Backend> GraphicsPipelineDesc<'a, B> {
//     /// Create a new empty PSO descriptor.
//     pub fn new(
//         shaders: GraphicsShaderSet<'a, B>,
//         primitive: Primitive,
//         rasterizer: Rasterizer,
//         layout: &'a B::PipelineLayout,
//         subpass: pass::Subpass<'a, B>,
//     ) -> Self {
//         GraphicsPipelineDesc {
//             shaders,
//             rasterizer,
//             vertex_buffers: Vec::new(),
//             attributes: Vec::new(),
//             input_assembler: InputAssemblerDesc::new(primitive),
//             blender: BlendDesc::default(),
//             depth_stencil: None,
//             layout,
//             subpass,
//             flags: PipelineCreationFlags::empty(),
//             parent: BasePipeline::None,
//         }
//     }
// }