///Interface for things that display/respond to log entries.
trait LogListener {
	///Called when a Logger has an entry for this listener to display.
	fn on_log(&self);
}