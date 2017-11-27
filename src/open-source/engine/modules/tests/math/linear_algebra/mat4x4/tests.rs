/*!
 Implementation of 4x4 matrix
 tests.
*/
use leek2::nearly_equal;
use leek2::math::Mat4x4;
use leek2::math::MatOps;
use std::ops::Range;
use super::internal::{generate_test_matrix, test_matrix_seed_range};
use super::internal::{multiplication_test_matrices, MulTestGroup};

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
	//Test:
	//Arithmetic scalar operators work:
	//divide, multiply, negation
	let scalars = [-2f32, -1f32, -0.5f32, 0f32, 0.5f32, 1f32, 2f32];
	for scalar_ref in &scalars {
		let scalar = *scalar_ref;
		for seed in test_matrix_seed_range() {
			let test_mat = generate_test_matrix(seed);

			let mut expected_quotient = test_mat;
			let mut expected_product = test_mat;
			let mut expected_negation = test_mat;
			for i in 0..16 {
				*expected_quotient.mut_elem_at(i) /= scalar;
				*expected_product.mut_elem_at(i) *= scalar;
				*expected_negation.mut_elem_at(i) = -expected_negation.elem_at(i);
			}

			let actual_quotient = test_mat / scalar;
			let actual_product = test_mat * scalar;
			let actual_negation = -test_mat;

			//Don't test divide by 0, since that's invalid under
			//all circumstances anyway
			if scalar != 0f32 {
				assert!(actual_quotient == expected_quotient, "{} / {} should == {}, returned {}", test_mat, scalar, expected_quotient, actual_quotient);
			}
			assert!(actual_product == expected_product, "{} * {} should == {}, returned {}", test_mat, scalar, expected_product, actual_product);
			assert!(actual_negation == expected_negation, "-{} should == {}, returned {}", test_mat, expected_negation, actual_negation);
		}
	}
}

#[test]
fn test_componentwise_operators() {
	//Test:
	//Arithmetic componentwise operators work:
	//addition, subtraction
	for seed1 in test_matrix_seed_range() {
		let test_mat1 = generate_test_matrix(seed1);
		for seed2 in test_matrix_seed_range() {
			let test_mat2 = generate_test_matrix(seed2);
		

			let mut expected_sum = test_mat1;
			let mut expected_difference = test_mat1;
			for i in 0..16 {
				*expected_sum.mut_elem_at(i) += test_mat2.elem_at(i);
				*expected_difference.mut_elem_at(i) -= test_mat2.elem_at(i);
			}

			let actual_sum = test_mat1 + test_mat2;
			let actual_difference = test_mat1 - test_mat2;

			assert!(actual_sum == expected_sum, "{} / {} should = {}, returned {}", test_mat1, test_mat2, expected_sum, actual_sum);
			assert!(actual_difference == expected_difference, "{} - {} should = {}, returned {}", test_mat1, test_mat2, expected_difference, actual_difference);
		}
	}
}

#[test]
fn test_multiplication() {
	//For all test groups:
	for test_group in multiplication_test_groups() {
		//	Test that:
		//		* A * B == AB
		let actual_m1m2 = test_group.m1 * test_group.m2;
		assert!(test_group.expected_m1m2 == actual_m1m2, "Matrix multiplication: {} * {} should == {}, but got {} instead", test_group.m1, test_group.m2, test_group.expected_m1m2, actual_m1m2);
		//		* B * A == BA
		let actual_m2m1 = test_group.m2 * test_group.m1;
		assert!(test_group.expected_m2m1 == actual_m2m1, "Matrix multiplication: {} * {} should == {}, but got {} instead", test_group.m2, test_group.m1, test_group.expected_m2m1, actual_m2m1);
		//	It's not necessarily true that AB != BA
		//	(identity, zero matrix), so don't assert for that
	}
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