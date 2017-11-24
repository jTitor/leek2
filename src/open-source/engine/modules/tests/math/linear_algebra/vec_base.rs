/*!
 Tests common vector functionality.
*/
use leek2::math::Vec3;
use leek2::math::VecOps;
use leek2::math::Vec2Access;
use leek2::math::scalar::nearly_equal;

#[test]
fn test_equality() {
	//Test:
	//Vector equality is reasonably close.
	let a = Vec3::new(1.0, 1.0, 1.0);
	let b = Vec3::new(1.0, 1.0, 1.0);

	assert!(a == b, "Vector equality failed");
}

#[test]
fn test_mutability() {
	//Test:
	//Mutable element access actually modifies
	//specified field
	let mut a = Vec3::new(1.0, 1.0, 1.0);
	const EXPECTED_VALUE: f64 = 2.0;
	*a.mut_x() = EXPECTED_VALUE as f32;

	assert!(nearly_equal(a.x() as f64, EXPECTED_VALUE), "Vector mutability failed");
}

#[test]
fn test_search_methods() {
	//Test:
	//minimum and maximum element methods work
	const EXPECTED_MIN: f64 = -4.0;
	const EXPECTED_MAX: f64 = 3.0;
	let a = Vec3::new(1.0, EXPECTED_MAX as f32, EXPECTED_MIN as f32);
	
	//For all nonzero vectors:
	//	* Minimum returns the smallest component
	//	in the vector
	assert!(nearly_equal(a.min_elem() as f64, EXPECTED_MIN), "Vector minimum failed to get actual minimum");
	//	* Maximum returns the largest component
	//	in the vector
	assert!(nearly_equal(a.max_elem() as f64, EXPECTED_MAX), "Vector maximum failed to get actual maximum");
}

#[test]
fn test_scalar_operators() {
	unimplemented!()
	//Test:
	//Arithmetic scalar operators work:
	//divide, multiply, negation
	//	For all vectors V, scalars x, and
	//	operations O(A, b), where A is a vector
	//	and b a scalar:
	//		* O(V, x) returns a vector
	//		<O(V.x, x), O(V.y, x), O(V.z, x)>,
	//		where o(a, b) is
	//		the scalar equivalent of O(A, B)
}

#[test]
fn test_componentwise_operators() {
	unimplemented!()
	//Test:
	//Arithmetic componentwise operators work:
	//addition, subtraction, multiplication, division
	//	For all vectors V, W and operations O(A, B):
	//		* O(V, W) returns a vector where all
	//		components c = o(V.c, W.c), where o(a, b)
	//		is the scalar equivalent of O(A, B)
	//Absolute value works
	//	For all vectors V:
	//		* abs(V) returns a vector where all
	//		components are >= 0
}

#[test]
fn test_geometric_methods() {
	unimplemented!()
	//Test:
	//dot product works
	//	* All vectors
	//magnitude works as expected
	//	* All nonzero vectors return a positive float
	//	that's not NaN
	//	* Zero vector returns length 0
	//square magnitude works
	//	* That is, all nonzero vectors tested return a
	//	positive float that's not NaN
	//	* Zero vector is still 0
	//normalization works
	//	* All nonzero vectors return
	//	a vector with *length* 1 and
	//	no components are NaN
	//	* Zero vector should return zero vector!!!
}