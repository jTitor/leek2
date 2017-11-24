/*!
 Tests functionality specific to 3-vectors.
*/
use leek2::math::Vec3;

#[test]
fn test_vec3_basis() {
	//Test:
	//All basis vectors are actually
	//length 1
	unimplemented!()
	
	//All basis vectors are zero in all
	//components that aren't their axis,
	//and are one in their axis component
	//unimplemented!()
}

#[test]
fn test_vec3_ops() {
	//Test:
	//3-vector methods work as expected:
	let v_up: Vec3 = Vec3::up();
	let v_right: Vec3 = Vec3::right();
	//Cross product
	unimplemented!();
	//Dot product
	//unimplemented!();
}