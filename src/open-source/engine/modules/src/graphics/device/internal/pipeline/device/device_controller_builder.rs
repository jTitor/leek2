/*!
 * Defines the DeviceControllerBuilder struct.
 */
use super::DeviceController;
use graphics::device::internal::pipeline as pipeline;

use std::marker::PhantomData;

use gfx_hal as hal;
use gfx_hal::{Device, Surface};
use gfx_hal::{format as f, pool as pool};
use failure::Error;

const ELEMENTS_PER_QUEUE: usize = 16;

/**
 * Handles creation of a DeviceController.
 */
pub struct DeviceControllerBuilder<'a, B: hal::Backend> {
	_backend_type: PhantomData<B>
}
impl<'a, B: hal::Backend> DeviceControllerBuilder<'a, B> {
	pub fn example(adapter: &hal::Adapter<B>, surface: &mut B::Surface, surface_format: f::Format) -> Result<DeviceController<'a, B>, Error> {
		let (device, mut queue_group) =
			adapter.open_with::<_, hal::Graphics>(1, |family| {
				surface.supports_queue_family(family)
			})?;

		let mut command_pool = device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty(), ELEMENTS_PER_QUEUE);
		let mut queue = &mut queue_group.queues[0];

		println!("Surface format: {:?}", surface_format);
		let swap_config = hal::SwapchainConfig::new()
			.with_color(surface_format);
		let (mut swap_chain, backbuffer) = device.create_swapchain(surface, swap_config);

		//TODO: viewport configuration from builder
		let viewport: pipeline::Viewport = Default::default();
		unimplemented!();

		Ok(DeviceController::<B> {
			resource_lists: Default::default(),
			device: device,
			command_pool: command_pool,
			queue_group: queue_group,
			main_queue: queue,
			swap_chain: swap_chain,
			backbuffer: backbuffer,
			viewport: viewport,
			frame_semaphore: device.create_semaphore(),
			//There's no documentation
			//currently on what a signaled fence
			//would do versus this; lookup the difference
			frame_fence: device.create_fence(false),
			//TODO_rust: make this an option
			frame_wait_timeout_ms: !0,
			frame_can_begin: true,
			resources_destroyed: false
		})
	}
}