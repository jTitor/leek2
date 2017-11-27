/*!
	Trait definitions for vector operations.
	This module represents 3-vectors.

	#Equality
	By default, vectors use nearly_equal in their comparison operations.
*/

//extern crate simd;
//use simd::f32x4;
use std::cmp;
use std::default::Default;
use std::fmt;
use std::ops;

use math;
pub use super::vec_base::{VecOps, Vec2Access, Vec3Access, Vec3Ops};

///Represents a 3-vector.
///A struct guaranteed to hold 3 f32s.
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
	pub data: [f32; 3]
}

impl VecOps<Vec3> for Vec3 {
	///Gets the i'th element of this vector.
	/// # Panics if:
	/// * i is out of range [0, num_elems()-1]
	fn elem_at(&self, i: usize) -> f32 {
		self.data[i]
	}
	
	fn mut_elem_at(&mut self, i: usize) -> &mut f32 {
		&mut self.data[i]
	}
	
	///Gets the squared magnitude of this vector.
	fn sqr_mag(&self) -> f32 {
		let mut result: f32 = 0.0;
		for i in 0..3 {
			result += self.data[i] * self.data[i];
		}
		result
	}

	///Performs the dot product between two vectors.
	///TODO: don't like how this dispatches - can we template on implementing type, or something?
	///Then we can be sure the underlying type is always the same.
	fn dot(&self, rhs: &Vec3) -> f32 {
		let mut result = 0.0;
		for i in 0..3 {
			result += self.data[i] * rhs.data[i];
		}
		result
	}
	
	///Returns a normalized version of the vector.
	///TODO: Again, can we have this return its underlying type?
	fn as_normalized(&self) -> Vec3 {
		let result = self.clone();
		if result.sqr_mag() > 0.0 { result / result.mag() } else { result }
	}
	///Returns this vector with all elements set to their absolute value.
	fn as_abs(&self) -> Vec3 {
		let mut result = self.clone();
		for i in 0..3 {
			result.data[i] = result.data[i].abs();
		}
		result
	}
	///Returns a vector with all elements set to their respective element's reciprocal.
	fn as_reciprocal(&self) -> Vec3 {
		let mut result = self.clone();
		for i in 0..3 {
			result.data[i] = 1.0 / result.data[i];
		}
		result
	}
	
	///Performs a componentwise multiplication.
	fn component_mul(&self, rhs: Vec3) -> Vec3 {
		let mut result = self.clone();
		for i in 0..3 {
			result.data[i] *= rhs.data[i];
		}
		result
	}

	///Performs a componentwise division.
	fn component_div(&self, rhs: Vec3) -> Vec3 {
		self.component_mul(rhs.as_reciprocal())
	}
	
	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32 {
		let mut result = self.data[0];
		for i in 0..3 {
			if self.data[i] > result {
				result = self.data[i];
			}
		}
		result
	}
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32 {
		let mut result = self.data[0];
		for i in 0..3 {
			if self.data[i] < result {
				result = self.data[i];
			}
		}
		result
	}
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
		let mut result: Vec3 = Default::default();
		*result.mut_x() = x;
		*result.mut_y() = y;
		*result.mut_z() = z;
		result
	}

	///Returns the 3-space up vector.
	pub fn up() -> Vec3 {
		Vec3::new(0.0, 1.0, 0.0)
	}

	///Returns the 3-space right vector.
	pub fn right() -> Vec3 {
		Vec3::new(1.0, 0.0, 0.0)
	}

	///Returns the 3-space forward vector.
	pub fn forward() -> Vec3 {
		//TODO: This might be really wrong!
		//Check that other libraries are RHS
		Vec3::new(0.0, 0.0, -1.0)
	}

	///Returns the 3-space down vector.
	pub fn down() -> Vec3 {
		-(Vec3::up())
	}

	///Returns the 3-space left vector.
	pub fn left() -> Vec3 {
		-(Vec3::right())
	}

	///Returns the 3-space back vector.
	pub fn back() -> Vec3 {
		-(Vec3::forward())
	}

	///Returns the 3-space zero vector.
	pub fn zero() -> Vec3 {
		Vec3::new(0.0, 0.0, 0.0)
	}
}

impl Vec2Access<Vec3> for Vec3 {}
impl Vec3Access<Vec3> for Vec3 {}

impl Vec3Ops<Vec3> for Vec3 {
	///Performs the cross product between two 3-vectors.
	fn cross(&self, rhs: &Vec3) -> Vec3 {
		let mut result: Vec3 = Default::default();
		result.data[0] = self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1];
		result.data[1] = self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2];
		result.data[2] = self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0];

		result
	}
}

//Begin operator implementation.

impl ops::Neg for Vec3 {
	type Output = Vec3;
	fn neg(self) -> Vec3 {
		let mut result: Vec3 = self;
		for i in 0..3 {
			result.data[i] = -result.data[i];
		}

		result
	}
}

impl ops::Add for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Vec3 {
		let mut result: Vec3 = self;
		for i in 0..3 {
			result.data[i] += rhs.data[i];
		}

		result
	}
}

impl ops::Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: Vec3) -> Vec3 {
		self + (-rhs)
	}
}

impl ops::Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f32) -> Vec3 {
		let mut result: Vec3 = self;
		for i in 0..3 {
			result.data[i] *= rhs;
		}

		result	
	}
}

impl ops::Mul<Vec3> for f32 {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Vec3 {
		rhs * self
	}
}

impl ops::Div<f32> for Vec3 {
	type Output = Vec3;

	fn div(self, rhs: f32) -> Vec3 {
		let mut result: Vec3 = self;
		for i in 0..3 {
			result.data[i] /= rhs;
		}

		result
	}
}

impl ops::Div<Vec3> for f32 {
	type Output = Vec3;

	fn div(self, rhs: Vec3) -> Vec3 {
		rhs / self
	}
}

impl cmp::PartialEq for Vec3 {
	fn eq(&self, other: &Vec3) -> bool {
		let mut result = true;
		for i in 0..3 {
			result = result && math::scalar::nearly_equal(self.data[i] as f64, other.data[i] as f64);
		}

		result
	}
}

impl Eq for Vec3 {}

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<{}, {}, {}>", self.x(), self.y(), self.z())
	}
}