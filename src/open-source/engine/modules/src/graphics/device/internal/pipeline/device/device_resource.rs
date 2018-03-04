/*!
 * Trait for resources managed
 * by a device controller.
 */
use std::rc::Weak;

pub trait DeviceResource<T> {
	/**
	 * Gets a new instance of the given resource.
	 */
	fn get_resource(&mut self) -> Weak<&T>;
	/**
	 * Destroys all of the resources
	 * of this type owned by the device.
	 */
	fn destroy_resources(&mut self);
	/**
	 * Destroys a single resource if
	 * it is owned by the device.
	 */
	fn destory_resource(&mut self, resource: &T);
}