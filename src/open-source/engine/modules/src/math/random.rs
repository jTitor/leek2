/*!
	PRNG operations.
*/
use rand;
use rand::distributions::{IndependentSample, Range};

pub struct Random {
	rng: rand::ThreadRng
}

impl Random {
	pub fn new() -> Random {
		Random {
			rng: rand::thread_rng()
		}
	}

	/**
	 * All ranges are inclusive on both ends.
	 */
	pub fn i64_in_range(&mut self, range_min: i64, range_max: i64) -> i64 {
		let range = Range::new(range_min, range_max);

		range.ind_sample(&mut self.rng)
	}

	pub fn u64_in_range(&mut self, range_min: u64, range_max: u64) -> u64 {
		let range = Range::new(range_min, range_max);
		
		range.ind_sample(&mut self.rng)
	}

	pub fn f64_in_range(&mut self, range_min: f64, range_max: f64) -> f64 {
		let range = Range::new(range_min, range_max);
		
		range.ind_sample(&mut self.rng)
	}

	pub fn f32_in_range(&mut self, range_min: f64, range_max: f64) -> f64 {
		let range = Range::new(range_min, range_max);
		
		range.ind_sample(&mut self.rng)
	}
}