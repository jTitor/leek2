/*!
 * Represents a screen-space rectangle.
 */
use super::Size;
use math::Vec2;

#[derive(Default)]
pub struct Rect {
	position: Vec2,
	dimensions: Size
}