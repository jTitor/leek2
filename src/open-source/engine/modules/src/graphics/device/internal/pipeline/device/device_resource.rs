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
pub trait DeviceResource<B: hal::Backend> where Self: Sized {
	//TODO: Device create/destroy calls don't
	//require mutability, update this
	/**
	 * Gets a new instance of the given resource.
	 */
	fn get_resource(device: &mut B::Device) -> Weak<&Self>;
	/**
	 * Destroys all of the resources
	 * of this type owned by the device.
	 */
	fn destroy_all_resources(device: &mut B::Device, resource_list: &Vec<Self>) -> Result<(), Error>;
	
	//Member methods.
	/**
	 * Destroys a single resource if
	 * it is owned by the device.
	 */
	fn destroy_resource(&mut self, device: &mut B::Device) -> Result<(), Error>;

	/**
	 * Destroys all resources in the collection.
	 */
	fn destroy_resource_collection<T: IntoIterator>(collection: &mut T, device: &mut B::Device) where T::Item == Self -> Result<(), Error> {
		for resource in collection {
			resource.destroy_resource(device)?;
		}

		Ok(())
	}

	/**
	 * If true, the resource does not need
	 * destroy_resource() to be called on it,
	 * since its allocated resources have
	 * already been destroyed.
	 */
	fn resources_destroyed(&self) -> bool;
}