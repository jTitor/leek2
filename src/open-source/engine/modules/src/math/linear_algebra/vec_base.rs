/*!
	Base traits for vector operations.
	Represents a generic vector.

	#Implementing Equality
	By default, vectors should use nearly_equal in their comparison operations.
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
	
	///Performs a componentwise multiplication.
	fn component_mul(&self, rhs: T) -> T;
	///Performs componentwise division.
	fn component_div(&self, rhs: T) -> T;
	
	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32;
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32;
}