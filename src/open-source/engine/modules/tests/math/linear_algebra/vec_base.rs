/*!
 Tests common vector functionality.
*/

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
fn test_search_methods() {
	unimplemented!()
	//Test:
	//minimum and maximum element methods work
	//For all nonzero vectors:
	//	* Minimum returns the smallest component
	//	in the vector
	//	* Maximum returns the largest component
	//	in the vector
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

#[test]
fn test_mutability() {
	unimplemented!()
	//Test:
	//Mutable element access actually modifies
	//specified field
}