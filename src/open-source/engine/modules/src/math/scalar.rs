/*!
 Handles scalar operations, such as float equality.
*/
use std::f64;

/**
Returns true if `a` and `b` are reasonably
equal according to floating point comparision.

In general, when testing floats for equality
you want to use this instead of ==, since
that only returns true when they're exactly the same.
 */
pub fn nearly_equal(a: f64, b: f64) -> bool {
	//Testing difference against absolute epsilon can't handle error between very small values;
	//see http://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
	//Instead, combine a quick epsilon test for differences very close to zero,
	//and a int reinterpretation (see http://randomascii.wordpress.com/2012/01/23/stupid-float-tricks-2/)
	//to find significant differences in all other situations.
	//Note that this relies on properties of IEEE floats.
	let abs_diff = (a - b).abs();
	if abs_diff <= f64::EPSILON
	{
		return true;
	}
	//If signs are different and the difference is past epsilon,
	//we can be pretty sure this isn't -0 and +0, so assume they're different.
	//Int reintepretation won't work when signs differ, as the sign bit is the MOST significant bit
	//and so the two values will appear to be billions of units apart.
	if a.signum() != b.signum()
	{
		return false;
	}
	//Otherwise, there's a fairly large difference between floats.
	//Reinterpret the floats as ints and get the integer difference to find how large that difference is
	//Specifically, the absolute value of the integer difference = 
	//1 + (number of representable floats between values)
	//since IEEE floats have mantissa as least significant bits,
	//and the exponent is the next significant bits past the mantissa
	let a_as_int = unsafe { *(&a as *const f64 as *const i64) };
	let b_as_int = unsafe { *(&b as *const f64 as *const i64) };
	let diff = a_as_int - b_as_int;
	//the numbers are approximately equal if diff is less than the maximum tolerance value
	const MAX_DIFF: i64 = 1;
	return diff.abs() < MAX_DIFF;
}

/**
From https://github.com/ajacksified/rusty/blob/master/src/libstd/sys/windows/time.rs
Computes (value*numer)/denom without overflow, as long as both
(numer*denom) and the overall result fit into i64 (which is the case
for our time conversions).
*/
pub fn mul_div_i64(value: i64, numer: i64, denom: i64) -> i64 {
	let q = value / denom;
	let r = value % denom;
	// Decompose value as (value/denom*denom + value%denom),
	// substitute into (value*numer)/denom and simplify.
	// r < denom, so (denom*numer) is the upper bound of (r*numer)
	q * numer + r * numer / denom
}