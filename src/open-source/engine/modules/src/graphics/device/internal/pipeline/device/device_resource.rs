/*!
 * Trait for resources managed
 * by a device controller.
 */
use std::rc::Weak;
use failure::Error;

use gfx_hal as hal;

pub trait DeviceResource<B: hal::Backend> where Self: Sized {
	/**
	 * Gets a new instance of the given resource.
	 */
	fn get_resource(device: &mut B::Device) -> Weak<&Self>;
	/**
	 * Destroys all of the resources
	 * of this type owned by the device.
	 */
	fn destroy_all_resources(device: &mut B::Device, resource_list: &Vec<Self>) -> Result<(), Error>;
	/**
	 * Destroys a single resource if
	 * it is owned by the device.
	 */
	fn destroy_resource(device: &mut B::Device, resource: &mut Self) -> Result<(), Error>;
}