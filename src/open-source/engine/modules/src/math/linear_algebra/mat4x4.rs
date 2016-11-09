//Represents a 4x4 matrix
//and common matrix operations.
use std::ops;

trait MatOps {
	fn elem_at(&self, i: usize) -> f32;
	fn mut_elem_at(&mut self, i: usize) -> &mut f32;

	fn to_index(row: usize, col: usize) -> usize;
}

///A 4x4 matrix.
pub struct Mat4x4 {
	pub data: [f32; 16]
}

impl MatOps for Mat4x4 {
	fn elem_at(&self, i: usize) -> f32 {
		self.data[i]
	}
	fn mut_elem_at(&mut self, i: usize) -> &mut f32 {
		&mut self.data[i]
	}

	fn to_index(row: usize, col: usize) -> usize {
		(row * 4) + col
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