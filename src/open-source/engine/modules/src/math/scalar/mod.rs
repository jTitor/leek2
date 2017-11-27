/*!
 Handles scalar operations, such as float equality.
*/
pub mod float_ops;
pub use self::float_ops::nearly_equal;

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