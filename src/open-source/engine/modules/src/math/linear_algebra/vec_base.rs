/*!
	Base traits for vector operations.
	Represents a generic vector.
*/
pub trait VecOps<T=Self> {
	///Gets the i'th element of this vector.
	/// # Panics if:
	/// * i is out of range [0, num_elems()-1]
	fn elem_at(&self, i: usize) -> f32;
	fn mut_elem_at(&mut self, i: usize) -> &mut f32;
	
	///Gets the squared magnitude of this vector.
	fn sqr_mag(&self) -> f32;
	///Gets the magnitude of this vector.
	fn mag(&self) -> f32 { self.sqr_mag().sqrt() }
	///Performs the dot product between two vectors.
	///TODO: don't like how this dispatches - can we template on implementing type, or something?
	///Then we can be sure the underlying type is always the same.
	fn dot(&self, rhs: &T) -> f32;
	
	///Returns a normalized version of the vector.
	///TODO: Again, can we have this return its underlying type?
	fn as_normalized(&self) -> T;
	///Returns this vector with all elements set to their absolute value.
	fn as_abs(&self) -> T;
	///Returns a vector with all elements set to their respective element's reciprocal.
	fn as_reciprocal(&self) -> T;
	
	///Performs a componentwise divtiplication.
	fn component_mul(&self, rhs: &T) -> T;
	///Performs componentwise division.
	fn component_div(&self, rhs: &T) -> T;
	
	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32;
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32;
}

///Represents access to 2 elements in a vector.
pub trait Vec2Access<T=Self> : VecOps<T> {
	fn x(&self) -> f32 { self.elem_at(0) }
	fn y(&self) -> f32 { self.elem_at(1) }
	fn mut_x(&mut self) -> &mut f32 { self.mut_elem_at(0) }
	fn mut_y(&mut self) -> &mut f32 { self.mut_elem_at(1) }
}

///Represents access to 3 elements in a vector.
pub trait Vec3Access<T=Self> : Vec2Access<T> {
	fn z(&self) -> f32 { self.elem_at(2) }
	fn mut_z(&mut self) -> &mut f32 { self.mut_elem_at(2) }
}

///Represents access to 4 elements in a vector.
pub trait Vec4Access<T=Self> : Vec3Access<T> {
	fn w(&self) -> f32 { self.elem_at(3) }
	fn mut_w(&mut self) -> &mut f32 { self.mut_elem_at(3) }
}