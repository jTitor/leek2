/*!
 * Defines the DeviceResource trait.
 */
use std::rc::Weak;
use failure::Error;

use gfx_hal as hal;

/**
 * Trait for resources managed
 * by a device controller.
 */
pub trait DeviceResource<B: hal::Backend> {
	/**
	 * Gets a new instance of the given resource.
	 */
	fn get_resource(device: &B::Device) -> Weak<&Self>;

	/**
	 * Destroys all resources in the collection.
	 */
	fn destroy_resource_collection<T: IntoIterator>(collection: &mut T, device: &B::Device) -> Result<(), Error> 
	where T::Item: DeviceResource<B> {
		for resource in collection {
			resource.destroy_resource(device)?;
		}

		Ok(())
	}
	
	//Member methods.
	/**
	 * Destroys a single resource if
	 * it is owned by the device.
	 */
	fn destroy_resource(&mut self, device: &B::Device) -> Result<(), Error>;

	/**
	 * If true, the resource does not need
	 * destroy_resource() to be called on it,
	 * since its allocated resources have
	 * already been destroyed.
	 */
	fn resources_destroyed(&self) -> bool;
}