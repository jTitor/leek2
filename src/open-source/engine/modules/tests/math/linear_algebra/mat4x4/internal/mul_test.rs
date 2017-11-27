/*!
 * Methods/structs associated with the multiplication test.
*/
use leek2::math::{Mat4x4, MatOps};

/**
 * Represents a single multiplication test:
 * the two matrices A and B to be multiplied,
 * their expected result A*B,
 * and their expected reversed result B*A.
*/
pub struct MulTestGroup {
	pub m1: Mat4x4,
	pub m2: Mat4x4,
	pub expected_m1m2: Mat4x4,
	pub expected_m2m1: Mat4x4
}

/// Returns a list of matrices used in
/// the multiplication test, and
/// their expected results when
/// multiplied forwards and backwards.
pub fn multiplication_test_groups() -> Vec<MulTestGroup> {
	let result = vec![
		//0 * 0 == 0
		MulTestGroup {
		m1: Mat4x4::identity(),
		m2: Mat4x4::zero(),
		expected_m1m2: Mat4x4::zero(),
		expected_m2m1: Mat4x4::zero() },
		//1 * 0 == 0, 0 * 1 == 0
		MulTestGroup {
		m1: Mat4x4::identity(),
		m2: Mat4x4::zero(),
		expected_m1m2: Mat4x4::zero(),
		expected_m2m1: Mat4x4::zero() },
		//1 * 1 == 1
		MulTestGroup {
		m1: Mat4x4::identity(),
		m2: Mat4x4::identity(),
		expected_m1m2: Mat4x4::identity(),
		expected_m2m1: Mat4x4::identity() },
		//A * 1 == A, 1 * A == A
		MulTestGroup {
		m1: Mat4x4::from_floats([17, 5, 2, 1, 1, 3, 3, 4, 9, 1, 1, 4, 7, 1, 5, 4]),
		m2: Mat4x4::identity(),
		expected_m1m2: Mat4x4::from_floats([17, 5, 2, 1, 1, 3, 3, 4, 9, 1, 1, 4, 7, 1, 5, 4]),
		expected_m2m1: Mat4x4::from_floats([17, 5, 2, 1, 1, 3, 3, 4, 9, 1, 1, 4, 7, 1, 5, 4]) },
		//A * B == AB, B * A == BA
		//Matrix values from misc/matrix_gen_literals.rs
		MulTestGroup {
		m1: Mat4x4::from_floats([17, 5, 2, 1, 1, 3, 3, 4, 9, 1, 1, 4, 7, 1, 5, 4]),
		m2: Mat4x4::from_floats([5, 4, 1, 3, 1, 4, 7, 7, 2, 7, 2, 4, 4, 4, 2, 8]),
		expected_m1m2: Mat4x4::from_floats([98, 106, 58, 102, 30, 53, 36, 68, 64, 63, 26, 70, 62, 83, 32, 80]),
		expected_m2m1: Mat4x4::from_floats([119, 41, 38, 37, 133, 31, 56, 73, 87, 37, 47, 54, 146, 42, 62, 60]) }];

	result
}