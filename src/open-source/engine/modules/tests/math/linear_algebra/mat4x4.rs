/*!
 Tests 4x4 matrices.
*/
use leek2::nearly_equal;
use leek2::math::Mat4x4;
use leek2::math::MatOps;

#[test]
fn test_access() {
	//Test:
	//to_index accesses expected cells
	const EXPECTED_DEFAULT_VALUE: f64 = 0.0;
	const DEFAULT_VALUE_IDX: usize = 6;

	let mut a = Mat4x4::new();
	let actual_default_value: f64 = a.elem_at(DEFAULT_VALUE_IDX) as f64;

	assert!(nearly_equal(actual_default_value, EXPECTED_DEFAULT_VALUE), "Default value at matrix index {} should be {}, is actually {}", DEFAULT_VALUE_IDX, EXPECTED_DEFAULT_VALUE, actual_default_value);
}

#[test]
fn test_search_methods() {
	//Test:
	//minimum and maximum element methods work
	const EXPECTED_MIN: f64 = -4.0;
	const EXPECTED_MAX: f64 = 3.0;
	let mut a = Mat4x4::new();
	*a.mut_elem_at(3) = EXPECTED_MIN as f32;
	*a.mut_elem_at(5) = EXPECTED_MAX as f32;
	
	//For all nonzero matrices:
	//	* Minimum returns the smallest component
	//	in the matrix
	let min = a.min_elem();
	let max = a.max_elem();
	assert!(nearly_equal(min as f64, EXPECTED_MIN), "Matrix minimum method failed to get actual minimum: should be {}, returned {}", EXPECTED_MIN, min);
	//	* Maximum returns the largest component
	//	in the matrix
	assert!(nearly_equal(max as f64, EXPECTED_MAX), "Matrix maximum method failed to get actual maximum: should be {}, returned {}", EXPECTED_MAX, max);
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
	//Test:
	//Mutable element access actually modifies
	//specified field
	const EXPECTED_VALUE1: f64 = -4.0;
	const EXPECTED_VALUE2: f64 = 3.0;
	const VALUE1_IDX: usize = 3;
	const VALUE2_IDX: usize = 5;

	let mut a = Mat4x4::new();

	*a.mut_elem_at(VALUE1_IDX) = EXPECTED_VALUE1 as f32;
	*a.mut_elem_at(VALUE2_IDX) = EXPECTED_VALUE2 as f32;
	
	let actual_value1: f64 = a.elem_at(VALUE1_IDX) as f64;
	let actual_value2: f64 = a.elem_at(VALUE2_IDX) as f64;
	assert!(nearly_equal(actual_value1, EXPECTED_VALUE1), "Value at matrix index {} should be {}, is actually {}", VALUE1_IDX, EXPECTED_VALUE1, actual_value1);

	assert!(nearly_equal(actual_value2, EXPECTED_VALUE2), "Value at matrix index {} should be {}, is actually {}", VALUE2_IDX, EXPECTED_VALUE2, actual_value2);
}