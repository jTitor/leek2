/*!
 * Unit tests for math::random module.
 */
use leek2::math::random;

/**
 * Tests that the random module actually produces random results.
 */
#[test]
fn test_random_is_random() {
	let mut rng = random::Random::new();
	let MIN_VAL: f32 = -10.0f32;
	let MAX_VAL: f32 = 10.0f32;
	let NUM_TO_TEST: i32 = 10000;
	let mut test: Vec<f32> = vec!();

	//Fill the test data
	for i in 0..NUM_TO_TEST {
		test.push(rng.f32_in_range(MIN_VAL as f64, MAX_VAL as f64) as f32);
	}

	//Now check the data:
	let num_under_min = test.iter().filter(|x: &&f32| **x < MIN_VAL).collect::<Vec<&f32>>().len();
	let num_over_max = test.iter().filter(|x: &&f32| **x > MAX_VAL).collect::<Vec<&f32>>().len();
	//Also check that the overall distribution is uniform.
	assert!(num_under_min == 0 && num_over_max == 0, "out of {} random values, {} values found under minimum {} and {} values over maximum {}", NUM_TO_TEST, num_under_min, MIN_VAL, num_over_max, MAX_VAL);

	let NUM_BUCKETS: i32 = 20;
	let BUCKET_SIZE = (MAX_VAL - MIN_VAL) / (NUM_BUCKETS as f32);
	let EXPECTED_FREQ: f32 = NUM_TO_TEST as f32 / NUM_BUCKETS as f32;
	let MAX_ACCEPTABLE_FREQ_RANGE: f32 = 0.2f32;
	let MIN_FREQ: f32 = EXPECTED_FREQ * (1f32 - MAX_ACCEPTABLE_FREQ_RANGE);
	let MAX_FREQ: f32 = EXPECTED_FREQ * (1f32 + MAX_ACCEPTABLE_FREQ_RANGE);
	//Now split these into buckets.
	let mut buckets: Vec<Vec<f32>> = vec!();
	let mut bucket_frequencies: Vec<f32> = vec!();
	for i in 0..NUM_BUCKETS {
		//Place elements into each bucket.
		let current_bucket_min: f32 = MIN_VAL + (BUCKET_SIZE * i as f32);
		let current_bucket_max: f32 = MIN_VAL + (BUCKET_SIZE * (i+1) as f32);
		assert!(current_bucket_min >= MIN_VAL && current_bucket_max <= MAX_VAL, "Bucket {} is out of range with min {} and max {}; global min is {} and global max is {}", i, current_bucket_min, current_bucket_max, MIN_VAL, MAX_VAL);

		let current_bucket: Vec<f32> = test.clone().into_iter().filter(|x: &f32| *x >= current_bucket_min && *x <= current_bucket_max).collect::<Vec<f32>>();
		let current_bucket_len = current_bucket.len(); 
		buckets.push(current_bucket);
		bucket_frequencies.push(current_bucket_len as f32);
	}

	let mut num_buckets_under_freq: i32 = 0i32;
	let mut num_buckets_over_freq: i32 = 0i32;
	for freq in bucket_frequencies.iter() {
		if *freq < MIN_FREQ {
			num_buckets_under_freq += 1;
		}
		if *freq > MAX_FREQ {
			num_buckets_over_freq += 1;
		}
	}

	assert!(num_buckets_under_freq == 0 && num_buckets_over_freq == 0, "{} of {} buckets are under minimum frequency {}, {} of {} are over maximum frequency {}", num_buckets_under_freq, bucket_frequencies.len(), MIN_FREQ, num_buckets_over_freq, bucket_frequencies.len(), MAX_FREQ);
}