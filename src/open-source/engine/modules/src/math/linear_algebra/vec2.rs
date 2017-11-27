/*!
	Trait definitions for vector operations.
	This module defines 2-vector component access
	and functions on 2-vectors.

	#Equality
	By default, vectors use nearly_equal in their comparison operations.
*/

//extern crate simd;
//use simd::f32x4;
pub use super::vec_base::VecOps;

///Represents access to 2 elements in a vector.
pub trait Vec2Access<T=Self> : VecOps<T> {
	fn x(&self) -> f32 { self.elem_at(0) }
	fn y(&self) -> f32 { self.elem_at(1) }
	fn mut_x(&mut self) -> &mut f32 { self.mut_elem_at(0) }
	fn mut_y(&mut self) -> &mut f32 { self.mut_elem_at(1) }
}

///Represents operations specific to a 2-vector.
pub trait Vec2Ops<T=Self> : VecOps<T> {
}