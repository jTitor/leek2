use leek2::time::{ClockFactory, Clock, TimeStamp};

#[test]
fn test_clock_update() {
	let mut clock = ClockFactory::new()
		.build().unwrap();
	clock.update();
	let prev_ts = clock.previous_timestamp();
	let prev_datetime = clock.previous_datetime();
	clock.update();
	let now_ts = clock.now_timestamp();
	let now_datetime = clock.now_datetime();
	
	assert!(now_ts > prev_ts,
		"Current timestamp after Clock.update() is `{}`, but should be later than previous timestamp `{}`",
		now_ts, prev_ts);
	assert!(now_datetime > prev_datetime,
		"Current date-time after Clock.update() is `{}`, but should be later than previous date-time `{}`",
		now_datetime, prev_datetime);
}

#[test]
fn test_clock_init() {
	//Test:
	let mut clock = ClockFactory::new()
		.build().unwrap();
	let now_ts = clock.now_timestamp();
	let prev_ts = clock.previous_timestamp();
	let expected_ts: TimeStamp = 0;
	
	assert!(now_ts == expected_ts,
		"Current timestamp immediately after ClockFactory.build is `{}`, expected {}",
		now_ts, expected_ts);
	assert!(prev_ts == expected_ts,
		"Previous timestamp immediately after ClockFactory.build is `{}`, expected {}",
		prev_ts, expected_ts);
}