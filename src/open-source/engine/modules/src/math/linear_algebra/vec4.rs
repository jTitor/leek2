/*!
	Trait definitions for vector operations.
	This module defines 4-vector component access
	and functions on 4-vectors.

	#Equality
	By default, vectors use nearly_equal in their comparison operations.
*/
use super::vec3::Vec3Access;
use super::vec_base::VecOps;

///Represents access to 4 elements in a vector.
pub trait Vec4Access<T=Self> : Vec3Access<T> {
	fn w(&self) -> f32 { self.elem_at(3) }
	fn mut_w(&mut self) -> &mut f32 { self.mut_elem_at(3) }
}

pub trait Vec4Ops<T=Self> : VecOps<T> where T: VecOps + Default {
}