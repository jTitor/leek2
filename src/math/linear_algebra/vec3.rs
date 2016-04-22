//Trait definitions for vector operations.

//extern crate simd;
//use simd::f32x4;

///Represents a generic vector.
trait Vector<T=Self> {
	///Gets the i'th element of this vector.
	/// # Panics if:
	/// * i is out of range [0, num_elems()-1]
	fn elem_at(&self, i: usize) -> f32;
	///Gets the number of valid elements this trait implementation contains.
	fn num_elems() -> u32;
	
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
	fn component_mul(&self, rhs: &T) -> T;
	///Performs componentwise division.
	fn component_div(&self, rhs: &T) -> T { self.component_mul(rhs.as_reciprocal()) }
	
	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32;
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32;
}

///Represents access to 2 elements in a vector.
trait Vec2Access<T=Self> : Vector<T> {
	fn x(&self) -> f32 { self.elem_at(0) }
	fn y(&self) -> f32 { self.elem_at(1) }
}

///Represents access to 3 elements in a vector.
trait Vec3Access<T=Self> : Vec2Access<T> {
	fn z(&self) -> f32 { self.elem_at(2) }
}

///Represents access to 4 elements in a vector.
trait Vec4Access<T=Self> : Vec3Access<T> {
	fn w(&self) -> f32 { self.elem_at(3) }
}

///Represents a 3-vector.
trait Vec3<T=Self> : Vector<T> {
	///Performs the cross product between two 3-vectors.
	fn cross(&self, rhs: &T) -> T;
}

///A struct guaranteed to hold 3 f32s.
#[derive(Debug, Copy, Clone)]
struct SIMDVec3 {
	data: [f32; 3]
}

impl Vector<SIMDVec3> for SIMDVec3 {
	///Gets the i'th element of this vector.
	/// # Panics if:
	/// * i is out of range [0, num_elems()-1]
	fn elem_at(&self, i: usize) -> f32 {
		self.data[i]
	}
	///Gets the number of valid elements this trait implementation contains.
	fn num_elems() -> u32 {
		3
	}
	
	///Gets the squared magnitude of this vector.
	fn sqr_mag(&self) -> f32 {
		let mut result = 0;
		for i in 0..3 {
			result += self.data[i] * self.data[i];
		}
		result
	}

	///Performs the dot product between two vectors.
	///TODO: don't like how this dispatches - can we template on implementing type, or something?
	///Then we can be sure the underlying type is always the same.
	fn dot(&self, rhs: &Vector<SIMDVec3>) -> f32 {
		let mut result = 0;
		for i in 0..3 {
			result += self.data[i] * rhs.data[i];
		}
		result
	}
	
	///Returns a normalized version of the vector.
	///TODO: Again, can we have this return its underlying type?
	fn as_normalized(&self) -> Vector<SIMDVec3> {
		let mut result = self.clone();
		result.component_div(result.mag())
	}
	///Returns this vector with all elements set to their absolute value.
	fn as_abs(&self) -> Vector<SIMDVec3> {
		let mut result = self.clone();
		for i in 0..3 {
			result.data[i] = result.data[i].abs();
		}
		result
	}
	///Returns a vector with all elements set to their respective element's reciprocal.
	fn as_reciprocal(&self) -> Vector<SIMDVec3> {
		let mut result = self.clone();
		for i in 0..3 {
			result.data[i] = 1.0 / result.data[i];
		}
		result
	}
	
	///Performs a componentwise multiplication.
	fn component_mul(&self, rhs: &Vector<SIMDVec3>) -> Vector<SIMDVec3> {
		let mut result = self.clone();
		for i in 0..3 {
			result.data[i] *= rhs.data[i];
		}
		result
	}
	
	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32 {
		let mut result = self.data[0];
		for i in 1..3 {
			if(self.data[i] > result) {
				result = self.data[i];
			}
		}
		result
	}
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32 {
		let result = self.data[0];
		for i in 1..3 {
			if(self.data[i] < result) {
				result = self.data[i];
			}
		}
		result
	}
	
	///Projects one vector onto another.
	///Note that neither vector need be normalized;
	///if you know that a destination is normalized,
	///use project_normalized().
	/*fn project(&self, destination: &Vector<SIMDVec3>) -> Vector<SIMDVec3> {
		projectedLen := self.dot(destination) / destination.sqr_mag();
		//Need to return destination * projectedLen, which doesn't exist yet.
	}
	///Projects one vector onto a normalized vector.
	fn project_normalized(&self, destination: &Vector<SIMDVec3>) -> Vector<SIMDVec3>;*/
}

impl Vec2Access<SIMDVec3> for SIMDVec3 {}
impl Vec3Access<SIMDVec3> for SIMDVec3 {}