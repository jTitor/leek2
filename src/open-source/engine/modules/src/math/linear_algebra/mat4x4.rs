/*!
	Represents a 4x4 matrix
	and common matrix operations.
*/

pub trait ToIndex {
	fn to_index(self) -> usize;
}

pub trait MatOps<T=Self> {
	fn elem_at<I: ToIndex>(&self, i: I) -> f32;
	fn mut_elem_at<I: ToIndex>(&mut self, i: I) -> &mut f32;

	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32;
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32;

	fn new() -> T;
}