/*!
	Trait definitions for vector operations.
	This module represents 2-vectors.
*/

//extern crate simd;
//use simd::f32x4;
use std::cmp;
use std::fmt;
use std::ops;

use math;
pub use super::vec_base::{VecOps, Vec2Access};

///Represents a 2-vector.
trait Vec2Ops<T=Self> : VecOps<T> {
}

///A struct guaranteed to hold 2 f32s.
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec2 {
	pub data: [f32; 2]
}

impl VecOps<Vec2> for Vec2 {
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
		for i in 0..2 {
			result += self.data[i] * self.data[i];
		}
		result
	}

	///Performs the dot product between two vectors.
	///TODO: don't like how this dispatches - can we template on implementing type, or something?
	///Then we can be sure the underlying type is always the same.
	fn dot(&self, rhs: &Vec2) -> f32 {
		let mut result = 0.0;
		for i in 0..2 {
			result += self.data[i] * rhs.data[i];
		}
		result
	}
	
	///Returns a normalized version of the vector.
	///TODO: Again, can we have this return its underlying type?
	fn as_normalized(&self) -> Vec2 {
		let result = self.clone();
		result / result.mag()
	}
	///Returns this vector with all elements set to their absolute value.
	fn as_abs(&self) -> Vec2 {
		let mut result = self.clone();
		for i in 0..2 {
			result.data[i] = result.data[i].abs();
		}
		result
	}
	///Returns a vector with all elements set to their respective element's reciprocal.
	fn as_reciprocal(&self) -> Vec2 {
		let mut result = self.clone();
		for i in 0..2 {
			result.data[i] = 1.0 / result.data[i];
		}
		result
	}
	
	///Performs a componentwise multiplication.
	fn component_mul(&self, rhs: &Vec2) -> Vec2 {
		let mut result = self.clone();
		for i in 0..2 {
			result.data[i] *= rhs.data[i];
		}
		result
	}
	///Performs a componentwise division.
	fn component_div(&self, rhs: &Vec2) -> Vec2 {
		self.component_mul(&rhs.as_reciprocal())
	}
	
	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32 {
		let mut result = self.data[0];
		for i in 0..2 {
			if self.data[i] > result {
				result = self.data[i];
			}
		}
		result
	}
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32 {
		let mut result = self.data[0];
		for i in 0..2 {
			if self.data[i] < result {
				result = self.data[i];
			}
		}
		result
	}
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2 {
		let mut result: Vec2 = Default::default();
		*result.mut_x() = x;
		*result.mut_y() = y;
		result
	}

	///Returns the 2-space up vector.
	pub fn up() -> Vec2 {
		Vec2::new(0.0, 1.0)
	}

	///Returns the 2-space right vector.
	pub fn right() -> Vec2 {
		Vec2::new(1.0, 0.0)
	}

	///Returns the 2-space down vector.
	pub fn down() -> Vec2 {
		-(Vec2::up())
	}

	///Returns the 2-space left vector.
	pub fn left() -> Vec2 {
		-(Vec2::right())
	}

	///Returns the 2-space zero vector.
	pub fn zero() -> Vec2 {
		Vec2::new(0.0, 0.0)
	}
}

impl Vec2Access<Vec2> for Vec2 {}

//Begin operator implementation.

impl ops::Neg for Vec2 {
	type Output = Vec2;
	fn neg(self) -> Vec2 {
		let mut result: Vec2 = self;
		for i in 0..2 {
			result.data[i] = -result.data[i];
		}

		result
	}
}

impl ops::Add for Vec2 {
	type Output = Vec2;

	fn add(self, rhs: Vec2) -> Vec2 {
		let mut result: Vec2 = self;
		for i in 0..2 {
			result.data[i] += rhs.data[i];
		}

		result
	}
}

impl ops::Sub for Vec2 {
	type Output = Vec2;

	fn sub(self, rhs: Vec2) -> Vec2 {
		self + (-rhs)
	}
}

impl ops::Mul<f32> for Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: f32) -> Vec2 {
		let mut result: Vec2 = self;
		for i in 0..2 {
			result.data[i] *= rhs;
		}

		result
	}
}

impl ops::Mul<Vec2> for f32 {
	type Output = Vec2;

	fn mul(self, rhs: Vec2) -> Vec2 {
		rhs * self
	}
}

impl ops::Div<f32> for Vec2 {
	type Output = Vec2;

	fn div(self, rhs: f32) -> Vec2 {
		let mut result: Vec2 = self;
		for i in 0..2 {
			result.data[i] /= rhs;
		}

		result
	}
}

impl ops::Div<Vec2> for f32 {
	type Output = Vec2;

	fn div(self, rhs: Vec2) -> Vec2 {
		rhs / self
	}
}

impl cmp::PartialEq for Vec2 {
	fn eq(&self, other: &Vec2) -> bool {
		let mut result = true;
		for i in 0..2 {
			result = result && math::scalar::nearly_equal(self.data[i] as f64, other.data[i] as f64);
		}

		result
	}
}

impl Eq for Vec2 {}

impl fmt::Display for Vec2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<{}, {}>", self.x(), self.y())
	}
}