/*!
	Represents a 4x4 matrix
	and common matrix operations.
*/

pub trait ToIndex {
	fn to_index(self) -> usize;
}

/**
Trait used to convert different matrix
representations to a 1D float array for
MatOps::from_floats().

#Expected Nontrivial Implementations
##Array of Arrays/Vectors ([[f32; 4]; 4], [Vec4; 4])
In these cases, the first index should be
by row, the second/vector component by column.
*/
pub trait ToMatrixArray {
	fn to_matrix_array(&self) -> [f32; 16];
}

pub trait MatOps<T=Self> {
	fn elem_at<I: ToIndex>(&self, i: I) -> f32;
	fn mut_elem_at<I: ToIndex>(&mut self, i: I) -> &mut f32;

	///Gets the maximum element in this vector.
	fn max_elem(&self) -> f32;
	///Gets the minimum element in this vector.
	fn min_elem(&self) -> f32;

	fn new() -> T;
	/**
	Converts a 1D array of 16 floats to a matrix array.
	Every 4 floats counts as one row of the matrix.
	*/
	fn from_floats<M>(floats: M) -> T where M: ToMatrixArray;
}