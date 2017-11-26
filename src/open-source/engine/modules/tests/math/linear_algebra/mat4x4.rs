/*!
 Tests 4x4 matrices.
*/
use leek2::math::scalar::nearly_equal;

#[test]
fn test_access() {
	unimplemented!()
	//Test:
	//to_index accesses expected cells
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
	assert!(nearly_equal(a.min_elem() as f64, EXPECTED_MIN), "Vector minimum method failed to get actual minimum");
	//	* Maximum returns the largest component
	//	in the vector
	assert!(nearly_equal(a.max_elem() as f64, EXPECTED_MAX), "Vector maximum method failed to get actual maximum");
}

#[test]
fn test_scalar_operators() {
	unimplemented!()
	//Test:
	//Arithmetic scalar operators work:
	//divide, multiply, negation
}

#[test]
fn test_componentwise_operators() {
	unimplemented!()
	//Test:
	//Arithmetic componentwise operators work:
	//addition, subtraction
}

#[test]
fn test_mutability() {
	unimplemented!()
	//Test:
	//Mutable element access actually modifies
	//specified field
}