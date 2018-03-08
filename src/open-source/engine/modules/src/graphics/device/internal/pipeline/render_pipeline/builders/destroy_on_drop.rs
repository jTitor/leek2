/*!
 * Defines the DestroyOnDrop wrapper structs.
 */
use graphics::device::internal::pipeline::DeviceResource;

use std::rc::Rc;

use gfx_hal as hal;

/**
 * If the given DeviceResource is dropped
 * without having its data freed,
 * its destroy_resource will be called.
 */
pub struct DestroyOnDrop<B: hal::Backend, T: DeviceResource<B>> {
	device: Rc<B::Device>,
	resource: T
}

impl<B: hal::Backend, T: DeviceResource<B>> DestroyOnDrop<B, T> {
	fn new(resource: T, device: &Rc<B::Device>) -> DestroyOnDrop {
		DestroyOnDrop::<B, T> {
			device: Rc::clone(device),
			resource: resource
		}
	}
}

impl<B: hal::Backend, T: DeviceResource<B>> Drop for DestroyOnDrop<B, T> {
	fn drop(&mut self) {
		if !self.resource.resources_destroyed() {
			self.resource.destroy_resource(self.device);
		}
	}
}

pub struct DestroyIterOnDrop<B, I> where
B: hal::Backend,
I: IntoIterator,
I::Item: DeviceResource<B> {
	device: Rc<B::Device>,
	resource_iter: T
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
}

impl<B, I> Drop for DestroyIterOnDrop<B, I> where
B: hal::Backend,
I: IntoIterator,
I::Item: DeviceResource<B> {
	fn drop(&mut self) {
		for resource in self.resource_iter {
			if !resource.resources_destroyed() {
				resource.destroy_resource(self.device);
			}
		}
	}
}