/*!
 Tests functionality specific to 3-vectors.
*/
use leek2::nearly_equal;
use leek2::math::Vec3;
use leek2::math::VecOps;
use leek2::math::Vec3Ops;

#[test]
fn test_vec3_basis() {
	//Test:
	//The zero vector has zero in all of its fields
	let zero = Vec3::zero();
	for i in 0..3 {
		let elem = zero.elem_at(i) as f64;
		assert!(nearly_equal(elem, 0.0f64), "Zero vector should be 0 in all fields, component {} was actually {}", i, elem);
	}

	//All basis vectors are zero in all
	//components that aren't their axis,
	//and are one in their axis component
	let basis_vectors_with_indices = [(Vec3::right(), 0), (Vec3::up(), 1), (Vec3::forward(), 2), (Vec3::left(), 0), (Vec3::down(), 1), (Vec3::back(), 2)];
	for basis_pair in &basis_vectors_with_indices {
		let &(basis_vector, basis_index) = basis_pair;

		for i in 0..3 {
			let actual_value = basis_vector.elem_at(i).abs() as f64;
			let expected_value = if i != basis_index { 0f64 } else { 1f64 };
			
			assert!(nearly_equal(actual_value, expected_value), "Value at index {} of basis vector {} should be {}, is actually {}", i, basis_vector, expected_value, actual_value);
		}
	}

	//All basis vectors are actually
	//length 1 when their magnitude is calculated
	for basis_pair in &basis_vectors_with_indices {
		let &(basis_vector, _) = basis_pair;
		const EXPECTED_MAG: f64 = 1f64;
		let actual_mag = basis_vector.mag() as f64;

		assert!(nearly_equal(actual_mag, EXPECTED_MAG), "Magnitude of basis vector {} should be {}, is actually {}", basis_vector, EXPECTED_MAG, actual_mag);
	}
}

#[test]
fn test_vec3_ops() {
	//Test:
	//3-vector methods work as expected:
	let x_vecs = [Vec3::right(), Vec3::left()];
	let y_vecs = [Vec3::up(), Vec3::down()];
	let z_vecs = [Vec3::forward(), Vec3::back()];

	//Dot product
	//Up and right are perpendicular, so they should
	//return a zero dot product
	const EXPECTED_DOT_PRODUCT: f64 = 0f64;
	for x in &x_vecs {
		for y in &y_vecs {
			for z in &z_vecs {
				let dot_products = [(x, y, x.dot(y)), (x, z, x.dot(z)), (y, x, y.dot(x)), (y, z, y.dot(z)), (z, x, z.dot(x)), (z, y, z.dot(y))];
				for product_tuple in &dot_products {
					let &(v1, v2, product) = product_tuple;
					assert!(nearly_equal(product as f64, EXPECTED_DOT_PRODUCT), "Dot product of vectors {} and {} should be {}, is actually {}", v1, v2, EXPECTED_DOT_PRODUCT, product);
				}
			}
		}
	}

	//Cross product
	let cross_test_tuples = [[Vec3::up(), Vec3::right(), Vec3::forward(), Vec3::back()], [Vec3::up(), Vec3::forward(), Vec3::left(), Vec3::right()], [Vec3::forward(), Vec3::right(), Vec3::down(), Vec3::up()]];

	for tuple in &cross_test_tuples {
		let v1 = tuple[0];
		let v2 = tuple[1];
		let expected_result = tuple[2];
		let expected_inverse_result = tuple[3];
		let actual_result = v1.cross(&v2);
		let actual_inverse_result = v2.cross(&v1);

		//v1 x v2 should == expected_result
		assert!(actual_result == expected_result, "{} x {} should == {}, is actually {}", v1, v2, expected_result, actual_result);
		
		//v2 x v1 should == -expected_result
		assert!(actual_inverse_result == -expected_result, "{} x {} should == {}, is actually {}", v2, v1, -expected_result, actual_inverse_result);
		
		//v1 x v2 should == -(v2 x v1)
		assert!(actual_result == -actual_inverse_result, "{} x {} should == -({} x {}), but ({} == -{}) returned false", v1, v2, v2, v1, actual_result, actual_inverse_result);

		//the specified negation should be
		//== -expected_result
		assert!(expected_result == -expected_inverse_result, "({} == -{}) should be true but returned false", expected_result, expected_inverse_result);
	}
}