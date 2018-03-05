/*!
 * Trait for resources managed
 * by a device controller.
 */
use std::rc::Weak;
use failure::Error;

pub trait DeviceResource<C> {
	/**
	 * Gets a new instance of the given resource.
	 */
	fn get_resource(&mut self) -> Weak<&C>;
	/**
	 * Destroys all of the resources
	 * of this type owned by the device.
	 */
	fn destroy_all_resources<T>(&mut self) -> Result<(), Error>;
	/**
	 * Destroys a single resource if
	 * it is owned by the device.
	 */
	fn destroy_resource<C>(&mut self, resource: &mut C) -> Result<(), Error>;
}