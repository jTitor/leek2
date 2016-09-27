//Defines the LogListener trait.
pub mod interface;

//Common error type.
pub mod listener_error;

//Implementations of LogListener.
pub mod file_listener;
pub mod terminal_listener;