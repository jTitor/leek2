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
	resource_val: T,
	was_unwrapped: bool
}

impl<B: hal::Backend, T: DeviceResource<B>> DestroyOnDrop<B, T> {
	fn new(resource: T, device: &Rc<B::Device>) -> DestroyOnDrop<B, T> {
		DestroyOnDrop::<B, T> {
			device: Rc::clone(device),
			resource_val: resource,
			was_unwrapped: false
		}
	}

	/**
	 * Provides access to the internal resource.
	 */
	fn resource(&self) -> &T {
		&self.resource_val
	}

	/**
	 * Provides mutable access to the internal resource.
	 */
	fn resource_mut(&self) -> &mut T {
		&mut self.resource_val
	}

	/**
	 * Returns the resource contained by this
	 * DestroyOnDrop wrapper and marks that
	 * the wrapper should not destroy
	 * its wrapped content.
	 */
	fn unwrap(&self) -> T {
		self.was_unwrapped = true;

		self.resource_val
	}
}

impl<B: hal::Backend, T: DeviceResource<B>> Drop for DestroyOnDrop<B, T> {
	fn drop(&mut self) {
		if !self.was_unwrapped &&
		!self.resource_val.resources_destroyed() {
			self.resource_val.destroy_resource(self.device);
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
	resource_iter_val: I,
	was_unwrapped: bool
}

impl<B, I> DestroyIterOnDrop<B, I> where
B: hal::Backend,
I: IntoIterator,
I::Item: DeviceResource<B> {
	fn new(resource_iter: I, device: &Rc<B::Device>) -> DestroyIterOnDrop<B, I> {
		DestroyIterOnDrop::<B, I> {
			device: Rc::clone(device),
			resource_iter_val: resource_iter,
			was_unwrapped: false,
		}
	}

	/**
	 * Provides access to the internal resource
	 * for iterable read operations.
	 */
	fn resource_iter(&self) -> &I {
		&self.resource_iter_val
	}

	/**
	 * Provides mutable access to the internal resource
	 * for operations such as appending to the
	 * iterable.
	 */
	fn resource_iter_mut(&self) -> &mut I {
		&mut self.resource_iter_val
	}

	/**
	 * Returns the resource contained by this
	 * DestroyOnDrop wrapper and marks that
	 * the wrapper should not destroy
	 * its wrapped content.
	 */
	fn unwrap(&self) -> I {
		self.was_unwrapped = true;

		self.resource_iter_val
	}
}

impl<B, I> Drop for DestroyIterOnDrop<B, I> where
B: hal::Backend,
I: IntoIterator,
I::Item: DeviceResource<B> {
	fn drop(&mut self) {
		for resource in self.resource_iter_val {
			if !self.was_unwrapped && !resource.resources_destroyed() {
				resource.destroy_resource(self.device);
			}
		}
	}
}