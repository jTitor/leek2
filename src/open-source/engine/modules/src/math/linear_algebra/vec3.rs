/*!
	Trait definitions for vector operations.
	This module defines 3-vector component access
	and functions on 3-vectors.

	#Equality
	By default, vectors use nearly_equal in their comparison operations.
*/

use super::vec_base::VecOps;
use super::vec2::Vec2Access;

///Represents access to 3 elements in a vector.
pub trait Vec3Access<T=Self> : Vec2Access<T> {
	fn z(&self) -> f32 { self.elem_at(2) }
	fn mut_z(&mut self) -> &mut f32 { self.mut_elem_at(2) }
}

pub trait Vec3Ops<T=Self> : VecOps<T> where T: VecOps + Default {
	///Performs the cross product between two 3-vectors.
	fn cross(&self, rhs: &T) -> T;
}