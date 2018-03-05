/*!
 * Handles creation of a DeviceController.
 */
use super::DeviceController;

use gfx_hal as hal;
use gfx_hal::pool as pool;

use failure::Error;

use std::rc::Rc;

const ELEMENTS_PER_QUEUE: u32 = 16;

pub struct DeviceControllerBuilder {}
impl DeviceControllerBuilder {
	pub fn example(surface: Rc<&hal::Surface>) -> Result<DeviceController, Error> {
		let (device, mut queue_group) =
			adapter.open_with::<_, hal::Graphics>(1, |family| {
				surface.supports_queue_family(family)
			})?;

		let mut command_pool = device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty(), ELEMENTS_PER_QUEUE);
		let mut queue = &mut queue_group.queues[0];

		println!("Surface format: {:?}", surface_format);
		let swap_config = hal::SwapchainConfig::new()
			.with_color(surface_format);
		let (mut swap_chain, backbuffer) = device.create_swapchain(&mut surface, swap_config);

		Ok(DeviceController {
			resource_lists: Default::default(),
			device: device,
			command_pool: command_pool,
			queue_group: queue_group,
			main_queue: queue,
			swap_chain: swap_chain,
			backbuffer: backbuffer,
			frame_semaphore: device.create_semaphore(),
			//There's no documentation
			//currently on what a signaled fence
			//would do versus this; lookup the difference
			frame_fence: device.create_fence(false),
			//TODO_rust: make this an option
			frame_wait_timeout_ms: !0,
			frame_can_begin: true
		})
	}
}