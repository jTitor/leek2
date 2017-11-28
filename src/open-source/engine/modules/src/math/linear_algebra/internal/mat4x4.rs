/*!
	Represents a 4x4 matrix
	and common matrix operations.
*/
use std::cmp;
use std::default::Default;
use std::fmt;
use std::ops;
use math;
use super::super::MatOps;
use super::super::mat4x4::{ToIndex, ToMatrixArray};
use num_traits::PrimInt;

///Indicator that a type can be converted to f32
///but is not itself f32
pub trait MatrixCellValue {}

impl MatrixCellValue for i8 {}
impl MatrixCellValue for i16 {}
impl MatrixCellValue for i32 {}
impl MatrixCellValue for i64 {}
impl MatrixCellValue for u8 {}
impl MatrixCellValue for u16 {}
impl MatrixCellValue for u32 {}
impl MatrixCellValue for u64 {}
impl MatrixCellValue for f64 {}

//ToIndex is implemented here so that
//we don't have to duplicate matrix_index!(),
//since it's supposed to be internal
//to this module
impl ToIndex for usize {
	fn to_index(self) -> usize {
		self
	}
}

macro_rules! matrix_index {
	//TODO: if multiplication test fails,
	//may need to reverse this
	($x:expr, $y:expr) => (($x*4) + $y);
}

impl ToIndex for (usize, usize) {
	fn to_index(self) -> usize {
		let (x, y) = self;
		debug_assert!(x < 4 && y < 4, "Matrix indexing: X and Y index need to both be < 4, but x = {} and y = {}", x, y);
		matrix_index!(x, y)
	}
}

impl ToMatrixArray for [f32; 16] {
	fn to_matrix_array(&self) -> [f32; 16] {
		*self
	}
}

impl<T> ToMatrixArray for [T; 16] where T: Into<f64> + Copy + Clone + MatrixCellValue, f64: From<T> {
	fn to_matrix_array(&self) -> [f32; 16] {
		let mut result: [f32; 16] = Default::default();
		for i in 0..16 {
			result[i] = f64::from(self[i]) as f32;
		}
		result
	}
}

///A 4x4 matrix.
#[derive(Debug, Copy, Clone, Default)]
pub struct Mat4x4 {
	pub data: [f32; 16]
}

impl MatOps<Mat4x4> for Mat4x4 {
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
		Default::default()
	}

	fn from_floats<M>(floats: M) -> Mat4x4 where M: ToMatrixArray {
		let mut result = Mat4x4::new();
		result.data = floats.to_matrix_array();

		result
	}
}

impl Mat4x4 {
	///Returns the zero matrix.
	pub fn zero() -> Mat4x4 {
		Mat4x4::new()
	}

	///Returns the identity matrix.
	pub fn identity() -> Mat4x4 {
		let mut result = Mat4x4::new();

		for i in 0..4 {
			*result.mut_elem_at((i, i)) = 1f32;
		}

		result
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

		result.data[matrix_index!(0, 0)] = self.data[matrix_index!(0, 0)] * rhs.data[matrix_index!(0, 0)] + self.data[matrix_index!(0, 1)] * rhs.data[matrix_index!(1, 0)] + self.data[matrix_index!(0, 2)] * rhs.data[matrix_index!(2, 0)] + self.data[matrix_index!(0, 3)] * rhs.data[matrix_index!(3, 0)];
		result.data[matrix_index!(0, 1)] = self.data[matrix_index!(0, 0)] * rhs.data[matrix_index!(0, 1)] + self.data[matrix_index!(0, 1)] * rhs.data[matrix_index!(1, 1)] + self.data[matrix_index!(0, 2)] * rhs.data[matrix_index!(2, 1)] + self.data[matrix_index!(0, 3)] * rhs.data[matrix_index!(3, 1)];
		result.data[matrix_index!(0, 2)] = self.data[matrix_index!(0, 0)] * rhs.data[matrix_index!(0, 2)] + self.data[matrix_index!(0, 1)] * rhs.data[matrix_index!(1, 2)] + self.data[matrix_index!(0, 2)] * rhs.data[matrix_index!(2, 2)] + self.data[matrix_index!(0, 3)] * rhs.data[matrix_index!(3, 2)];
		result.data[matrix_index!(0, 3)] = self.data[matrix_index!(0, 0)] * rhs.data[matrix_index!(0, 3)] + self.data[matrix_index!(0, 1)] * rhs.data[matrix_index!(1, 3)] + self.data[matrix_index!(0, 2)] * rhs.data[matrix_index!(2, 3)] + self.data[matrix_index!(0, 3)] * rhs.data[matrix_index!(3, 3)];
		result.data[matrix_index!(1, 0)] = self.data[matrix_index!(1, 0)] * rhs.data[matrix_index!(0, 0)] + self.data[matrix_index!(1, 1)] * rhs.data[matrix_index!(1, 0)] + self.data[matrix_index!(1, 2)] * rhs.data[matrix_index!(2, 0)] + self.data[matrix_index!(1, 3)] * rhs.data[matrix_index!(3, 0)];
		result.data[matrix_index!(1, 1)] = self.data[matrix_index!(1, 0)] * rhs.data[matrix_index!(0, 1)] + self.data[matrix_index!(1, 1)] * rhs.data[matrix_index!(1, 1)] + self.data[matrix_index!(1, 2)] * rhs.data[matrix_index!(2, 1)] + self.data[matrix_index!(1, 3)] * rhs.data[matrix_index!(3, 1)];
		result.data[matrix_index!(1, 2)] = self.data[matrix_index!(1, 0)] * rhs.data[matrix_index!(0, 2)] + self.data[matrix_index!(1, 1)] * rhs.data[matrix_index!(1, 2)] + self.data[matrix_index!(1, 2)] * rhs.data[matrix_index!(2, 2)] + self.data[matrix_index!(1, 3)] * rhs.data[matrix_index!(3, 2)];
		result.data[matrix_index!(1, 3)] = self.data[matrix_index!(1, 0)] * rhs.data[matrix_index!(0, 3)] + self.data[matrix_index!(1, 1)] * rhs.data[matrix_index!(1, 3)] + self.data[matrix_index!(1, 2)] * rhs.data[matrix_index!(2, 3)] + self.data[matrix_index!(1, 3)] * rhs.data[matrix_index!(3, 3)];
		result.data[matrix_index!(2, 0)] = self.data[matrix_index!(2, 0)] * rhs.data[matrix_index!(0, 0)] + self.data[matrix_index!(2, 1)] * rhs.data[matrix_index!(1, 0)] + self.data[matrix_index!(2, 2)] * rhs.data[matrix_index!(2, 0)] + self.data[matrix_index!(2, 3)] * rhs.data[matrix_index!(3, 0)];
		result.data[matrix_index!(2, 1)] = self.data[matrix_index!(2, 0)] * rhs.data[matrix_index!(0, 1)] + self.data[matrix_index!(2, 1)] * rhs.data[matrix_index!(1, 1)] + self.data[matrix_index!(2, 2)] * rhs.data[matrix_index!(2, 1)] + self.data[matrix_index!(2, 3)] * rhs.data[matrix_index!(3, 1)];
		result.data[matrix_index!(2, 2)] = self.data[matrix_index!(2, 0)] * rhs.data[matrix_index!(0, 2)] + self.data[matrix_index!(2, 1)] * rhs.data[matrix_index!(1, 2)] + self.data[matrix_index!(2, 2)] * rhs.data[matrix_index!(2, 2)] + self.data[matrix_index!(2, 3)] * rhs.data[matrix_index!(3, 2)];
		result.data[matrix_index!(2, 3)] = self.data[matrix_index!(2, 0)] * rhs.data[matrix_index!(0, 3)] + self.data[matrix_index!(2, 1)] * rhs.data[matrix_index!(1, 3)] + self.data[matrix_index!(2, 2)] * rhs.data[matrix_index!(2, 3)] + self.data[matrix_index!(2, 3)] * rhs.data[matrix_index!(3, 3)];
		result.data[matrix_index!(3, 0)] = self.data[matrix_index!(3, 0)] * rhs.data[matrix_index!(0, 0)] + self.data[matrix_index!(3, 1)] * rhs.data[matrix_index!(1, 0)] + self.data[matrix_index!(3, 2)] * rhs.data[matrix_index!(2, 0)] + self.data[matrix_index!(3, 3)] * rhs.data[matrix_index!(3, 0)];
		result.data[matrix_index!(3, 1)] = self.data[matrix_index!(3, 0)] * rhs.data[matrix_index!(0, 1)] + self.data[matrix_index!(3, 1)] * rhs.data[matrix_index!(1, 1)] + self.data[matrix_index!(3, 2)] * rhs.data[matrix_index!(2, 1)] + self.data[matrix_index!(3, 3)] * rhs.data[matrix_index!(3, 1)];
		result.data[matrix_index!(3, 2)] = self.data[matrix_index!(3, 0)] * rhs.data[matrix_index!(0, 2)] + self.data[matrix_index!(3, 1)] * rhs.data[matrix_index!(1, 2)] + self.data[matrix_index!(3, 2)] * rhs.data[matrix_index!(2, 2)] + self.data[matrix_index!(3, 3)] * rhs.data[matrix_index!(3, 2)];
		result.data[matrix_index!(3, 3)] = self.data[matrix_index!(3, 0)] * rhs.data[matrix_index!(0, 3)] + self.data[matrix_index!(3, 1)] * rhs.data[matrix_index!(1, 3)] + self.data[matrix_index!(3, 2)] * rhs.data[matrix_index!(2, 3)] + self.data[matrix_index!(3, 3)] * rhs.data[matrix_index!(3, 3)];

		result
	}
}

impl fmt::Display for Mat4x4 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {}]", self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7], self.data[8], self.data[9], self.data[10], self.data[11], self.data[12], self.data[13], self.data[14], self.data[15])
	}
}