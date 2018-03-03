/*!
 * Trait for resources managed
 * by a device controller.
 */
use std::rc::Weak;

pub trait DeviceResource<T> {
	fn get_resource(&mut self) -> Weak<&T>;
	fn destroy_resource(&mut self, resource: &T);
}