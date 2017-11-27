/*!
	Represents a 4x4 matrix
	and common matrix operations.
*/
use std::cmp;
use std::default;
use std::fmt;
use std::ops;
use math;

pub trait ToIndex {
	fn to_index(self) -> usize;
}

impl ToIndex for usize {
	fn to_index(self) -> usize {
		self
	}
}

impl ToIndex for (usize, usize) {
	fn to_index(self) -> usize {
		let (x, y) = self;
		debug_assert!(x < 4 && y < 4, "Matrix indexing: X and Y index need to both be < 4, but x = {} and y = {}", x, y);
		//TODO: if multiplication test fails,
		//may need to reverse this
		(y*4) + x
	}
}

pub trait MatOps {
	fn elem_at<T: ToIndex>(&self, i: T) -> f32;
	fn mut_elem_at<T: ToIndex>(&mut self, i: T) -> &mut f32;

	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32;
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32;

	fn new() -> Mat4x4;

	///Returns the identity matrix.
	fn identity() -> Mat4x4 {
		let mut result: Mat4x4 = Default::default();

		for i in 0..4 {
			*result.mut_elem_at((i as usize, i as usize)) = 1f32;
		}

		result
	}

	///Returns the zero matrix.
	fn zero() -> Mat4x4 {
		let mut result: Mat4x4 = Default::default();
		result
	}
}

///A 4x4 matrix.
#[derive(Debug, Copy, Clone, Default)]
pub struct Mat4x4 {
	pub data: [f32; 16]
}

impl MatOps for Mat4x4 {
	fn elem_at<T: ToIndex>(&self, i: T) -> f32 {
		self.data[i.to_index()]
	}
	fn mut_elem_at<T: ToIndex>(&mut self, i: T) -> &mut f32 {
		&mut self.data[i.to_index()]
	}

	fn max_elem(&self) -> f32 {
		let mut result = self.data[0];
		for i in 0..16 {
			if self.data[i] > result {
				result = self.data[i];
			}
		}
		
		result
	}

	fn min_elem(&self) -> f32 {
		let mut result = self.data[0];
		for i in 0..16 {
			if self.data[i] < result {
				result = self.data[i];
			}
		}
		result
	}

	fn new() -> Mat4x4 {
		Mat4x4 {
			data: [0.0; 16]
		}
	}
}

//Operator implementations.
impl ops::Neg for Mat4x4 {
	type Output = Mat4x4;
	fn neg(self) -> Mat4x4 {
		let mut result: Mat4x4 = self;
		for i in 0..16 {
			result.data[i] = -result.data[i];
		}

		result
	}
}

impl ops::Add for Mat4x4 {
	type Output = Mat4x4;

	fn add(self, rhs: Mat4x4) -> Mat4x4 {
		let mut result: Mat4x4 = self;
		for i in 0..16 {
			result.data[i] += rhs.data[i];
		}

		result
	}
}

impl ops::Sub for Mat4x4 {
	type Output = Mat4x4;

	fn sub(self, rhs: Mat4x4) -> Mat4x4 {
		self + (-rhs)
	}
}

impl ops::Mul<f32> for Mat4x4 {
	type Output = Mat4x4;

	fn mul(self, rhs: f32) -> Mat4x4 {
		let mut result: Mat4x4 = self;
		for i in 0..16 {
			result.data[i] *= rhs;
		}

		result	
	}
}

impl ops::Mul<Mat4x4> for f32 {
	type Output = Mat4x4;

	fn mul(self, rhs: Mat4x4) -> Mat4x4 {
		rhs * self
	}
}

impl ops::Div<f32> for Mat4x4 {
	type Output = Mat4x4;

	fn div(self, rhs: f32) -> Mat4x4 {
		let mut result: Mat4x4 = self;
		for i in 0..16 {
			result.data[i] /= rhs;
		}

		result	
	}
}

impl ops::Div<Mat4x4> for f32 {
	type Output = Mat4x4;

	fn div(self, rhs: Mat4x4) -> Mat4x4 {
		rhs / self
	}
}

impl cmp::PartialEq for Mat4x4 {
	fn eq(&self, other: &Mat4x4) -> bool {
		let mut result = true;
		for i in 0..16 {
			result = result && math::scalar::nearly_equal(self.data[i] as f64, other.data[i] as f64);
		}

		result
	}
}

impl Eq for Mat4x4 {}

impl ops::Mul<Mat4x4> for Mat4x4 {
	type Output = Mat4x4;

	fn mul(self, rhs: Mat4x4) -> Mat4x4 {
		let mut result: Mat4x4 = Default::default();



		result
	}
}

impl fmt::Display for Mat4x4 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {}]", self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7], self.data[8], self.data[9], self.data[10], self.data[11], self.data[12], self.data[13], self.data[14], self.data[15])
	}
}