/*!
 * Defines traits for debug primitives.
 */
use math::Transform;

pub enum DebugRenderLevel {
	Error,
	Warning,
	Info,
	Verbose
}

pub type DebugRenderTag = &str;

pub trait DebugPrimitive {
	fn transform(&self) -> Transform;
	fn level -> DebugRenderLevel;
	fn tag -> DebugRenderTag;
}