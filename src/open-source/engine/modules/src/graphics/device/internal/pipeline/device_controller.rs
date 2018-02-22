/*!
 * Provides implementation-independent
 * access to the device's draw calls.
 */

fn example() {
	let (device, mut queue_group) =
		adapter.open_with::<_, hal::Graphics>(1, |family| {
			surface.supports_queue_family(family)
		}).unwrap();

	let mut command_pool = device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty(), 16);
	let mut queue = &mut queue_group.queues[0];

	println!("Surface format: {:?}", surface_format);
	let swap_config = SwapchainConfig::new()
		.with_color(surface_format);
	let (mut swap_chain, backbuffer) = device.create_swapchain(&mut surface, swap_config);
}

pub struct DeviceController {
	
}