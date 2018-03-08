/*!
 * Contains non-user-facing but platform-independent
 * structs and functions.
 * 
 * As far as the library user's concerned,
 * the only important parts are that:
 *   * A Device represents a graphics rendering backend
 *   * The Device can have vertex data uploaded
 *   * The Device can have image data uploaded
 *   * The Device can be given shaders to render with
 *   * When so asked, the Device can render provided
 *     data
 * 
 * This module describes how the Device does all of
 * these tasks. Structures and methods that aren't publicly
 * facing can reference implementation structures/values.
 */

pub mod buffer;
pub use self::buffer::{BufferType, BufferUploadType, BufferBuilder, ElementBufferBuilder, ImageBufferBuilder, MemoryBuffer, WriteableBuffer};

pub mod device;
pub use self::device::{DeviceController, DeviceControllerBuilder, DeviceInfo, Viewport, DeviceResource};

pub mod render_pipeline;
pub use self::render_pipeline::{RenderPipeline, DefaultPipelineBuilder, RenderPipelineBuilder};

mod descriptor_pool;
pub use self::descriptor_pool::DescriptorPool;