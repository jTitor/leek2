/*!
 Tests common vector functionality.
*/
use leek2::math::Vec3;
use leek2::math::VecOps;
use leek2::math::Vec2Access;
use leek2::math::scalar::nearly_equal;
use std::ops;

#[test]
#[should_panic]
fn test_invalid_access() {
	//Test:
	//Accessing out of range element panics.
	let a = Vec3::new(1.0, 1.0, 1.0);
	println!("{}", a.elem_at(4));
}

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
	assert!(nearly_equal(a.min_elem() as f64, EXPECTED_MIN), "Vector minimum method failed to get actual minimum");
	//	* Maximum returns the largest component
	//	in the vector
	assert!(nearly_equal(a.max_elem() as f64, EXPECTED_MAX), "Vector maximum method failed to get actual maximum");
}

#[test]
fn test_scalar_operators() {
	//Test:
	//Arithmetic scalar operators work:
	//divide, multiply, negation
	let scalars = [-2f32, -1f32, -0.5f32, 0f32, 0.5f32, 1f32, 2f32];
	for scalar_ref in &scalars {
		let scalar = *scalar_ref;
		for x in -1i16..2i16 {
			for y in -1i16..2i16 {
				for z in -1i16..2i16 {
					let test_vec = Vec3::new(f32::from(x), f32::from(y), f32::from(z));
					//	For all vectors V, scalars x, and
					//	operations O(A, b), where A is a vector
					//	and b a scalar:
					//		* O(V, x) returns a vector
					//		<O(V.x, x), O(V.y, x), O(V.z, x)>,
					//		where o(a, b) is
					//		the scalar equivalent of O(A, B)
					let mut expected_quotient = test_vec;
					let mut expected_product = test_vec;
					let mut expected_negation = test_vec;
					for i in 0..3 {
						*expected_quotient.mut_elem_at(i) /= scalar;
						*expected_product.mut_elem_at(i) *= scalar;
						*expected_negation.mut_elem_at(i) = -expected_negation.elem_at(i);
					}

					let actual_quotient = test_vec / scalar;
					let actual_product = test_vec * scalar;
					let actual_negation = -test_vec;

					//Don't test divide by 0, since that's invalid under
					//all circumstances anyway
					if scalar != 0f32 {
						assert!(actual_quotient == expected_quotient, "{} / {} should == {}, returned {}", test_vec, scalar, expected_quotient, actual_quotient);
					}
					assert!(actual_product == expected_product, "{} * {} should == {}, returned {}", test_vec, scalar, expected_product, actual_product);
					assert!(actual_negation == expected_negation, "-{} should == {}, returned {}", test_vec, expected_negation, actual_negation);
				}
			}
		}
	}
}

//Componentwise operations.
#[test]
fn test_absolute_value() {
	//Absolute value works
	const MIN_COMPONENT_VALUE: f64 = 0.0f64;
	//	For all vectors V:
	for x in -1i16..2i16 {
		for y in -1i16..2i16 {
			for z in -1i16..2i16 {
				let test_vec = Vec3::new(f32::from(x), f32::from(y), f32::from(z)).as_abs();

				//Test that abs(V) returns a vector where all
				//components are >= 0
				for i in 0..3 {
					let actual_component = test_vec.elem_at(i) as f64;
					assert!(actual_component >= MIN_COMPONENT_VALUE, "Absolute value of {}.elem_at({}) should be >= {}, returned {}", test_vec, i, MIN_COMPONENT_VALUE, actual_component);
				}
			}
		}
	}
}

#[test]
fn test_componentwise_operators() {
	//Test:
	//Arithmetic componentwise operators work:
	//addition, subtraction, multiplication, division
	//	For all vectors V, W and operations O(A, B):
	//		* O(V, W) returns a vector where all
	//		components c = o(V.c, W.c), where o(a, b)
	//		is the scalar equivalent of O(A, B)

	//...definitely stayed up too late
	for v1_x in -1i16..2i16 {
		for v1_y in -1i16..2i16 {
			for v1_z in -1i16..2i16 {
				for v2_x in -1i16..2i16 {
					for v2_y in -1i16..2i16 {
						for v2_z in -1i16..2i16 {
							let test_vec1 = Vec3::new(f32::from(v1_x), f32::from(v1_y), f32::from(v1_z));
							let test_vec2 = Vec3::new(f32::from(v2_x), f32::from(v2_y), f32::from(v2_z));
							let mut divisor_has_zero = false;

							let mut expected_sum = test_vec1;
							let mut expected_difference = test_vec1;
							let mut expected_quotient = test_vec1;
							let mut expected_product = test_vec1;
							for i in 0..3 {
								divisor_has_zero |= test_vec2.elem_at(i) == 0f32;

								*expected_sum.mut_elem_at(i) += test_vec2.elem_at(i);
								*expected_difference.mut_elem_at(i) -= test_vec2.elem_at(i);
								*expected_quotient.mut_elem_at(i) /= test_vec2.elem_at(i);
								*expected_product.mut_elem_at(i) *= test_vec2.elem_at(i);
							}

							let actual_sum = test_vec1 + test_vec2;
							let actual_difference = test_vec1 - test_vec2;
							let actual_quotient = test_vec1.component_div(test_vec2);
							let actual_product = test_vec1.component_mul(test_vec2);

							assert!(actual_sum == expected_sum, "{} / {} should = {}, returned {}", test_vec1, test_vec2, expected_sum, actual_sum);
							assert!(actual_difference == expected_difference, "{} - {} should = {}, returned {}", test_vec1, test_vec2, expected_difference, actual_difference);
							//Again, skip test for division if any divisor component is 0
							if !divisor_has_zero {
								assert!(actual_quotient == expected_quotient, "{} / {} should = {}, returned {}", test_vec1, test_vec2, expected_quotient, actual_quotient);
							}
							assert!(actual_product == expected_product, "{} * {} should = {}, returned {}", test_vec1, test_vec2, expected_product, actual_product);
						}
					}
				}
			}
		}
	}
}

//Geometric operations.
fn nonzero_geometric_vecs() -> Vec<Vec3> {
	vec![Vec3::up(), Vec3::down(), Vec3::left(), Vec3::right(), Vec3::forward(), Vec3::back(), Vec3::new(1.0, 2.0, 3.0), Vec3::new(-4.0, -5.0, -6.0), Vec3::new(7.0, -8.0, -9.0)]
}

#[test]
fn test_dot_product() {
	let zero_vec: Vec3 = Vec3::zero();
	let mut all_vecs = nonzero_geometric_vecs();
	all_vecs.push(zero_vec);
	//Test:
	//dot product works
	//	* All vectors
	for i in 0..all_vecs.len() {
		let mut j = (i + 1) % all_vecs.len();
		while i != j {
			let vec_a = all_vecs[i];
			let vec_b = all_vecs[j];
			let mut expected_dot: f32 = 0.0;
			for x in 0..3 {
				expected_dot += vec_a.elem_at(x) * vec_b.elem_at(x);
			}
			let actual_dot: f32 = vec_a.dot(&vec_b);
			let actual_dot_reverse: f32 = vec_b.dot(&vec_a);
			assert!(nearly_equal(actual_dot as f64, actual_dot_reverse as f64), "Dot product isn't transitive for {} and {}; a.b is {}, b.a is {}", vec_a, vec_b, actual_dot, actual_dot_reverse);
			assert!(nearly_equal(actual_dot as f64, expected_dot as f64), "Dot product for {} and {} should be {}, got {}", vec_a, vec_b, expected_dot, actual_dot);

			j = (j + 1) % all_vecs.len();
		}
	}
}

#[test]
fn test_magnitude() {
	let zero_vec: Vec3 = Vec3::zero();
	let nonzero_vecs = nonzero_geometric_vecs();
	//magnitude works as expected
	//	* All nonzero vectors return a positive float
	//	that's not NaN
	for v in nonzero_vecs {
		let v_mag = v.mag();
		assert!(v_mag > 0.0, "Magnitude of nonzero vector should be > 0, was {}", v_mag);
		assert!(!v_mag.is_nan(), "Magnitude of nonzero vector should not be NaN");
	}
	//	* Zero vector returns length 0
	let zero_mag = zero_vec.mag() as f64;
	assert!(nearly_equal(zero_mag, 0.0), "Magnitude of zero vector should be 0, was {}", zero_mag);
}

#[test]
fn test_square_magnitude() {
	let zero_vec: Vec3 = Vec3::zero();
	let nonzero_vecs = nonzero_geometric_vecs();

	//square magnitude works
	//	* That is, all nonzero vectors tested return a
	//	positive float that's not NaN
	for v in nonzero_vecs {
		let v_sqr_mag = v.sqr_mag();
		assert!(v_sqr_mag > 0.0, "Square magnitude of nonzero vector should be > 0, was {}", v_sqr_mag);
		assert!(!v_sqr_mag.is_nan(), "Square magnitude of nonzero vector should not be NaN");
	}
	//	* Zero vector is still 0
	let zero_sqr_mag = zero_vec.sqr_mag() as f64;
	assert!(nearly_equal(zero_sqr_mag, 0.0), "Square magnitude of zero vector should be 0, was {}", zero_sqr_mag);
}

#[test]
fn test_normalization() {
	let zero_vec: Vec3 = Vec3::zero();
	let nonzero_vecs = nonzero_geometric_vecs();

	//normalization works
	//	* All nonzero vectors return
	//	a vector with *length* 1 and
	//	no components are NaN
	for v in nonzero_vecs {
		let mut v_normalized = v.as_normalized();
		let first_normalization: Vec3 = v_normalized;
		let mut normalized_mag = v_normalized.mag();
		let mut num_times_normalized = 1;
		const MAX_TIMES_NORMALIZED: u64 = 2;
		let mut normalized = nearly_equal(normalized_mag, 1.0);
		while !normalized && num_times_normalized < MAX_TIMES_NORMALIZED {
			v_normalized = v_normalized.as_normalized();
			normalized_mag = v_normalized.mag();
			normalized = nearly_equal(normalized_mag, 1.0);
			num_times_normalized += 1;
		}
		//A vector should be length 1 after a single normalization...
		if num_times_normalized > 1 {
			warn!("Vector {} was normalized {} times", v, num_times_normalized);
		}
		assert!(normalized && num_times_normalized <= MAX_TIMES_NORMALIZED, "Magnitude of normalized vector should be 1, but failed to normalize after {} tries. Magnitude after {} normalization(s) was {}; original vector is {}, first normalized is {}, final normalized is {}", num_times_normalized, num_times_normalized, normalized_mag, v, first_normalization, v_normalized);
	}
	//	* Zero vector should return zero vector!!!
	assert!(nearly_equal(zero_vec.as_normalized().sqr_mag() as f64, 0.0), "Zero vector should normalize to zero vector");
}