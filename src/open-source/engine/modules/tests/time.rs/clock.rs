use leek2::(ClockFactory, Clock, Timestamp);

#[test]
fn test_clock_update() {
	let clock: mut Clock = ClockFactory::new()
		.build().unwrap();
	clock.update();
	let now_ts = clock.now_timestamp();
	let prev_ts = clock.previous_timestamp();
	assert!(now_ts > prev_ts,
		"Current timestamp after Clock.update() is `{}`, but should be later than previous timestamp `{}`",
		now_ts, prev_ts);

	let now_datetime = clock.now_datetime();
	let prev_datetime = clock.prev_datetime();
}

#[test]
fn test_clock_init() {
	unimplemented!()
	//Test:
	let clock: mut Clock = ClockFactory::new()
		.build().unwrap();
	let now_ts = clock.now_timestamp();
	let prev_ts = clock.previous_timestamp();
	let expected_ts: Timestamp = 0;
	assert!(now_ts == expected_ts,
		"Current timestamp immediately after ClockFactory.build is `{}`, expected {}",
		now_ts, expected_ts);
	assert!(prev_ts == expected_ts,
		"Previous timestamp immediately after ClockFactory.build is `{}`, expected {}",
		prev_ts, expected_ts);
}