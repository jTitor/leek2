/*!
 * Contains modules describing the
 * subpasses'
 * external and internal layout.
 */

mod subpass_description;
pub use self::subpass_description::SubpassDescription;

pub use gfx_hal::pass::SubpassDependency as SubpassDependency;