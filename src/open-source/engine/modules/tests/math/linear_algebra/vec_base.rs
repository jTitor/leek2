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
	let zero_vec: Vec3 = Vec3::zero();
	let nonzero_vecs = [Vec3::up(), Vec3::down(), Vec3::left(), Vec3::right(), Vec3::forward(), Vec3::back(), Vec3::new(1.0, 2.0, 3.0), Vec3::new(-4.0, -5.0, -6.0), Vec3::new(7.0, -8.0, -9.0)];
	let mut all_vecs = nonzero_vecs.to_vec();
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
	//magnitude works as expected
	//	* All nonzero vectors return a positive float
	//	that's not NaN
	for v in &nonzero_vecs {
		let v_mag = v.mag();
		assert!(v_mag > 0.0, "Magnitude of nonzero vector should be > 0, was {}", v_mag);
		assert!(!v_mag.is_nan(), "Magnitude of nonzero vector should not be NaN");
	}
	//	* Zero vector returns length 0
	let zero_mag = zero_vec.mag() as f64;
	assert!(nearly_equal(zero_mag, 0.0), "Magnitude of zero vector should be 0, was {}", zero_mag);
	//square magnitude works
	//	* That is, all nonzero vectors tested return a
	//	positive float that's not NaN
	for v in &nonzero_vecs {
		let v_sqr_mag = v.sqr_mag();
		assert!(v_sqr_mag > 0.0, "Square magnitude of nonzero vector should be > 0, was {}", v_sqr_mag);
		assert!(!v_sqr_mag.is_nan(), "Square magnitude of nonzero vector should not be NaN");
	}
	//	* Zero vector is still 0
	let zero_sqr_mag = zero_vec.sqr_mag() as f64;
	assert!(nearly_equal(zero_sqr_mag, 0.0), "Square magnitude of zero vector should be 0, was {}", zero_sqr_mag);
	//normalization works
	//	* All nonzero vectors return
	//	a vector with *length* 1 and
	//	no components are NaN
	for v in &nonzero_vecs {
		let normalized_mag = v.as_normalized().mag();
		assert!(nearly_equal(normalized_mag as f64, 1.0), "Magnitude of normalized vector should be 1, was {}", normalized_mag);
	}
	//	* Zero vector should return zero vector!!!
	assert!(nearly_equal(zero_vec.as_normalized().sqr_mag() as f64, 0.0), "Zero vector should normalize to zero vector");
}