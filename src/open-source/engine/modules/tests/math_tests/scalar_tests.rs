/*!
 Unit tests for scalar methods.
*/
use std::f64;
use leek2::math::scalar;

#[test]
fn test_nearly_equal() {
	const POSITIVE: f64 = 3.0;
	const NEGATIVE: f64 = -2.0;
	const POS_ZERO: f64 = 0.0;
	const NEG_ZERO: f64 = -0.0;
	const POS_INF: f64 = f64::INFINITY;
	const NEG_INF: f64 = f64::NEG_INFINITY;
	const NAN: f64 = f64::NAN;

	let mut positive_as_int = unsafe { *(&POSITIVE as *const f64 as *const i64) };
	let mut negative_as_int = unsafe { *(&NEGATIVE as *const f64 as *const i64) };
	positive_as_int += 1;
	negative_as_int += 1;

	let near_positive: f64 = unsafe { *(&positive_as_int as *const i64 as *const f64) };
	let near_negative: f64 = unsafe { *(&negative_as_int as *const i64 as *const f64) };
	const FAR_POSITIVE: f64 = POSITIVE + 0.5;
	const FAR_NEGATIVE: f64 = NEGATIVE - 0.5;

	//This should match
	let should_match = [[POSITIVE, POSITIVE], [POS_ZERO, NEG_ZERO], [POSITIVE, near_positive], [NEGATIVE, near_negative]];

	for pair in &should_match {
		let a = pair[0];
		let b = pair[1];
		assert!(scalar::nearly_equal(a, b), "{} and {} should be equal under scalar::nearly_equal()", a, b);
	}

	//This should NOT match
	let should_not_match = [[POSITIVE, NEGATIVE], [POS_INF, NEG_INF], [NAN, NAN], [POSITIVE, near_negative], [NEGATIVE, near_positive], [POSITIVE, POS_ZERO], [POSITIVE, NEG_ZERO], [POSITIVE, POS_INF], [POSITIVE, NEG_INF], [NEGATIVE, POS_ZERO], [NEGATIVE, NEG_ZERO], [NEGATIVE, POS_INF], [NEGATIVE, NEG_INF], [POSITIVE, NAN], [NEGATIVE, NAN], [POSITIVE, FAR_POSITIVE], [NEGATIVE, FAR_NEGATIVE], [POSITIVE, FAR_NEGATIVE], [NEGATIVE, FAR_POSITIVE]];

	for pair in &should_not_match {
		let a = pair[0];
		let b = pair[1];
		assert!(!scalar::nearly_equal(a, b), "{} and {} should not be equal under scalar::nearly_equal()", a, b);
	}

	//see if we can avoid short-circuiting?
}