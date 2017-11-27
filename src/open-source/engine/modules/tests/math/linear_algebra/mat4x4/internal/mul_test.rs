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
	m1: Mat4x4,
	m2: Mat4x4,
	expected_m1m2: Mat4x4,
	expected_m2m1: Mat4x4
}

/// Returns a list of matrices used in
/// the multiplication test, and
/// their expected results when
/// multiplied forwards and backwards.
fn multiplication_test_matrices() -> Vec<MulTestGroup> {
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
		expected_m2m1: Mat4x4::identity() }
		//TODO: A * 1 == A, 1 * A == A
		//TODO: A * B == AB, B * A == BA
		 ];

	unimplemented!("Not all test groups have been defined")
	result
}