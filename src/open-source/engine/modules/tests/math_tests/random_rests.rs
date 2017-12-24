/*!
 * Unit tests for math::random module.
 */
use leek2::math::random;

/**
 * Tests that the random module actually produces random results.
 */
[test]
fn test_random_is_random() {
	let rng = random::Random::new();
	let MIN_VAL = -10.0f32;
	let MAX_VAL = 10.0f32;
	let NUM_TO_TEST = 10000;
	let mut test: Vec<f32> = vec!();

	//Fill the test data
	for i in 0..NUM_TO_TEST {
		test.push(rng.f32_in_range(MIN_VAL, MAX_VAL));
	}

	//Now check the data:
	let num_under_min = test.filter(|x| x < MIN_VAL).count();
	let num_over_max = test.filter(|x| x > MAX_VAL).count();
	//Also check that the overall distribution is uniform.
	assert!(num_under_min == 0 && num_over_max == 0, "out of {} random values, {} values found under minimum {} and {} values over maximum {}", NUM_TO_TEST, num_under_min, MIN_VAL, num_over_max, MAX_VAL);

	let NUM_BUCKETS = 20;
	let BUCKET_SIZE = (MAX_VAL - MIN_VAL) / (NUM_BUCKETS as f32);
	let EXPECTED_FREQ = NUM_TO_TEST / NUM_BUCKETS
	let MAX_ACCEPTABLE_FREQ_RANGE = 0.2f32;
	let MIN_FREQ = EXPECTED_FREQ * (1f32 - MAX_ACCEPTABLE_FREQ_RANGE);
	let MAX_FREQ = EXPECTED_FREQ * (1f32 + MAX_ACCEPTABLE_FREQ_RANGE);
	//Now split these into buckets.
	let mut buckets: Vec<Vec<f32>> = vec!();
	let mut bucket_frequencies: Vec<f32> = vec!();
	for i in 0..NUM_BUCKETS {
		//Place elements into each bucket.
		let current_bucket_min = MIN_VAL + (BUCKET_SIZE * i);
		let current_bucket_max = MIN_VAL + (BUCKET_SIZE * (i+1));
		assert!(current_bucket_min >= MIN_VAL && current_bucket_max <= MAX_VAL, "Bucket {} is out of range with min {} and max {}; global min is {} and global max is {}", i, current_bucket_min, current_bucket_max, MIN_VAL, MAX_VAL);

		let current_bucket: Vec<f32> = test.filter(|x| x >= current_bucket_min && x <= current_bucket_max);
		buckets.push(current_bucket);
		bucket_frequencies.push(current_bucket.count());
	}

	let mut num_buckets_under_freq = 0u32;
	for freq in bucket_frequencies {
		if freq >= MIN_FREQ && freq <= MAX_FREQ {
			num_buckets_under_freq += 1;
		}
	}

	assert!(num_buckets_under_freq == 0, "{} of buckets are outside of frequency range [{}, {}]", (bucket_frequencies.count() as f32) / (num_buckets_under_freq as f32), MIN_FREQ, MAX_FREQ);
}