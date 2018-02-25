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
 * these tasks.
 */

pub mod buffer;
pub use self::buffer::{FileBuffer, MemoryBuffer};

pub mod device;

pub mod render_pipeline;

pub mod image;

pub mod render_target;

pub mod sampler;