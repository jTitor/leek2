/*!
 * Contains modules describing the
 * subpasses'
 * external and internal layout.
 */

mod subpass_description;
pub use self::subpass_description::SubpassDescription;

mod subpass_dependency;
pub use self::subpass_dependency::SubpassDependency;