/*!
 * Defines the DestroyOnDrop wrapper structs.
 */
use graphics::device::internal::pipeline::DeviceResource;

use std::rc::Rc;

use gfx_hal as hal;

/**
 * If the given DeviceResource is dropped
 * without having its data freed or retrieved
 * via unwrap(),
 * its destroy_resource() will be called.
 */
pub struct DestroyOnDrop<B: hal::Backend, T: DeviceResource<B>> {
	device: Rc<B::Device>,
	resource: T,
	was_unwrapped: bool
}

impl<B: hal::Backend, T: DeviceResource<B>> DestroyOnDrop<B, T> {
	fn new(resource: T, device: &Rc<B::Device>) -> DestroyOnDrop {
		DestroyOnDrop::<B, T> {
			device: Rc::clone(device),
			resource: resource
		}
	}

	/**
	 * Returns the resource contained by this
	 * DestroyOnDrop wrapper and marks that
	 * the wrapper should not destroy
	 * its wrapped content.
	 */
	fn unwrap(&self) -> T {
		self.was_unwrapped = true;

		resource
	}
}

impl<B: hal::Backend, T: DeviceResource<B>> Drop for DestroyOnDrop<B, T> {
	fn drop(&mut self) {
		if !self.was_unwrapped &&
		!self.resource.resources_destroyed() {
			self.resource.destroy_resource(self.device);
		}
	}
}

/**
 * Version of DestroyOnDrop that wraps
 * iterable groups of DeviceResources.
 * When this is dropped, each element
 * of its wrapped iterable will have
 * destroy_resource() called on it.
 */
pub struct DestroyIterOnDrop<B, I> where
B: hal::Backend,
I: IntoIterator,
I::Item: DeviceResource<B> {
	device: Rc<B::Device>,
	resource_iter: T,
	was_unwrapped: bool
}

impl<B, I> DestroyIterOnDrop<B, I> where
B: hal::Backend,
I: IntoIterator,
I::Item: DeviceResource<B> {
	fn new(resource_iter: T, device: &Rc<B::Device>) -> DestroyIterOnDrop {
		DestroyIterOnDrop::<B, T> {
			device: Rc::clone(device),
			resource_iter: resource_iter
		}
	}

	/**
	 * Returns the resource contained by this
	 * DestroyOnDrop wrapper and marks that
	 * the wrapper should not destroy
	 * its wrapped content.
	 */
	fn unwrap(&self) -> T {
		self.was_unwrapped = true;

		resource
	}
}

impl<B, I> Drop for DestroyIterOnDrop<B, I> where
B: hal::Backend,
I: IntoIterator,
I::Item: DeviceResource<B> {
	fn drop(&mut self) {
		for resource in self.resource_iter {
			if !self.was_unwrapped && !resource.resources_destroyed() {
				resource.destroy_resource(self.device);
			}
		}
	}
}