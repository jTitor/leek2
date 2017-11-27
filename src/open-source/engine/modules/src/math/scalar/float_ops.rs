use std::f64;
use num_traits::Float;

/**
Returns true if `a` and `b` are reasonably
equal according to floating point comparision.

In general, when testing floats for equality
you want to use this instead of ==, since
that only returns true when they're exactly the same.
 */
pub fn nearly_equal<T: Into<f64> + Float>(a: T, b: T) -> bool where f64: From<T> {
	//Testing difference against absolute epsilon can't handle error between very small values;
	//see http://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
	//Instead, combine a quick epsilon test for differences very close to zero,
	//and a int reinterpretation (see http://randomascii.wordpress.com/2012/01/23/stupid-float-tricks-2/)
	//to find significant differences in all other situations.
	//Note that this relies on properties of IEEE floats.
	let abs_diff = (a - b).abs();
	let epsilon_small_enough = f64::from(abs_diff) <= f64::EPSILON;

	//If signs are different and the difference is past epsilon,
	//we can be pretty sure this isn't -0 and +0, so assume they're different.
	//Int reintepretation won't work when signs differ, as the sign bit is the MOST significant bit
	//and so the two values will appear to be billions of units apart.
	//NaN should fail here, since NaN != NaN
	let signs_differ = a.signum() != b.signum();

	//Try to early out here if possible,
	//since the interpret test can overflow if the numbers are too big.
	if epsilon_small_enough || signs_differ {
		return epsilon_small_enough || !signs_differ;
	}

	//Otherwise, there's a fairly large difference between floats.
	//Reinterpret the floats as ints and get the integer difference to find how large that difference is
	//Specifically, the absolute value of the integer difference = 
	//1 + (number of representable floats between values)
	//since IEEE floats have mantissa as least significant bits,
	//and the exponent is the next significant bits past the mantissa
	let a_as_int: i64 = unsafe { *(&a as *const T as *const i64) };
	let b_as_int: i64 = unsafe { *(&b as *const T as *const i64) };
	let diff: i64 = a_as_int - b_as_int;
	//the numbers are approximately equal if diff is less than the maximum tolerance value
	const MAX_DIFF: i64 = 1;
	let interpret_differs = diff.abs() <= MAX_DIFF;

	epsilon_small_enough || (!signs_differ && interpret_differs)
}