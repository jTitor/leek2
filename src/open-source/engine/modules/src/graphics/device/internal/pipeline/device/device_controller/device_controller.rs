/*!
 * Defines the DeviceController struct.
 */
use super::DeviceResourceLists;
use graphics::device::internal::pipeline::{DeviceResource, MemoryBuffer, RenderPipeline};
use graphics::device::internal::pipeline::render_pipeline::elements::{Image, Sampler, RenderTarget};

use gfx_hal as hal;
use gfx_hal::{Device, Swapchain, Graphics};
use gfx_hal::pool::RawCommandPool;
use gfx_hal::{command, pool, pso};
use gfx_hal::command::{CommandBuffer, Submit, Primary};
use gfx_hal::pso::PipelineStage;
use gfx_hal::queue::Submission;
use gfx_hal::queue::capability as capability;

/**
 * Provides implementation-independent
 * access to the device's draw calls.
 */
pub struct DeviceController<B: hal::Backend> {
	resource_lists: DeviceResourceLists<B>,

	device: B::Device,
	command_pool: pool::CommandPool<B, hal::Graphics>,
	//problem here - maybe this has to be external to the struct?
	queue_group: hal::QueueGroup<B, hal::Graphics>,
	main_queue: hal::CommandQueue<B, hal::Graphics>,
	swap_chain: B::Swapchain,
	//TODO: don't keep as a field -
	//this is meant to be unwrapped into a RenderTarget
	backbuffer: hal::Backbuffer<B>,

	viewport: command::Viewport,

	frame_semaphore: B::Semaphore,
	frame_fence: B::Fence,
	frame_wait_timeout_ms: u32,

	frame_can_begin: bool,

	resources_destroyed: bool
}

impl<'a, B: hal::Backend> DeviceController<B> {
	/**
	 * Readies the device for a draw submission.
	 */
	fn begin_frame(&mut self) {
		debug_assert!(self.frame_can_begin, "begin_frame() already called before this begin_frame() call");

		self.device.reset_fence(&self.frame_fence);
		self.command_pool.reset();

		unimplemented!();

		self.frame_can_begin = false;
	}

	/**
	 * Completes a draw submission
	 * and updates all render targets this device
	 * is operating on.
	 */
	fn end_frame(&mut self) {
		debug_assert!(!self.frame_can_begin, "begin_frame() wasn't called before end_frame()");
		//Wait for the fence so we
		//know the buffer's finished updating.
		//TODO_rust: the timeout in a real
		//app shouldn't be infinity
		self.device.wait_for_fence(&self.frame_fence,
			self.frame_wait_timeout_ms);// !0);

		self.swap_chain.present(&mut self.main_queue, &[]);

		// #[cfg(feature = "metal")]
		// unsafe {
		// 	autorelease_pool.reset();
		// }
		unimplemented!();

		self.frame_can_begin = true;
	}

	// fn submit(&mut self, submission: ?) {
	// 	debug_assert!(!self.frame_can_begin, "begin_frame() wasn't called before this submit() call");

	// 	let submission = Submission::new()
	// 		.wait_on(&[(&frame_semaphore, PipelineStage::BOTTOM_OF_PIPE)])
	// 		.submit(Some(submit));
	// 	queue.submit(submission, Some(&mut frame_fence));
	// }

	pub fn upload_to_buffer<I, C, S>(&mut self, submit: I) where I: command::Submittable<'a, B, S, command::Primary>, (hal::Transfer, S): capability::Upper {
		let submission = Submission::new()
			.submit(Some(submit));
		self.main_queue.submit(submission, Some(&mut self.frame_fence));

		self.device.wait_for_fence(&self.frame_fence, self.frame_wait_timeout_ms);
	}

	/**
	 * Performs rendering with the given pipeline.
	 */
	pub fn render_with_pipeline(&mut self, pipeline: &RenderPipeline<B>) {
		self.begin_frame();
		//Ask the pipeline for a submission given a command buffer.
		let mut cmd_buffer = self.command_pool.acquire_command_buffer(false);
		let submission = pipeline.submission_with_cmd_buffer(cmd_buffer);
		self.end_frame();
	}

	fn REMOVE_submit(&mut self, pipeline: &RenderPipeline<B>, render_pass: &B::RenderPass, framebuffers: &Vec<B::Framebuffer>, vertex_buffer: &B::Buffer, desc_set: &B::DescriptorSet) {
		//The submission used to draw the
		//demo scene.
		//TODO_rust: figure out how to *actually*
		//split this up
		let frame = self.swap_chain.acquire_frame(hal::FrameSync::Semaphore(&mut self.frame_semaphore));

		let submit = {
			let mut cmd_buffer = self.command_pool.acquire_command_buffer(false);
			let viewport = self.viewport as command::Viewport;
			let pipeline = self.resource_lists.pipelines[0];

			cmd_buffer.set_viewports(&[viewport.clone()]);
			cmd_buffer.set_scissors(&[viewport.rect]);
			cmd_buffer.bind_graphics_pipeline(&pipeline.pipeline_impl.as_ref().unwrap());
			cmd_buffer.bind_vertex_buffers(pso::VertexBufferSet(vec![(vertex_buffer, 0)]));
			cmd_buffer.bind_graphics_descriptor_sets(&pipeline.pipeline_layout, 0, Some(desc_set)); //TODO

			{
				let mut encoder = cmd_buffer.begin_render_pass_inline(
					render_pass,
					&framebuffers[frame.id()],
					viewport.rect,
					&[command::ClearValue::Color(command::ClearColor::Float([0.8, 0.8, 0.8, 1.0]))],
				);
				encoder.draw(0..6, 0..1);
			}

			cmd_buffer.finish()
		};

		let submission = Submission::new()
			.wait_on(&[(&self.frame_semaphore, pso::PipelineStage::BOTTOM_OF_PIPE)])
			.submit(Some(submit));
		self.main_queue.submit(submission, Some(&mut self.frame_fence));
	}

	pub fn destroy_resources(&mut self) {
		//TODO: Have this be manually dropped
		//to specify order?
		self.device.destroy_command_pool(self.command_pool.downgrade());
		//device.destroy_descriptor_pool(desc_pool);
		//device.destroy_descriptor_set_layout(set_layout);

		//Destroy all the unpacked framebuffers.
		//Destroy all the resources!
		MemoryBuffer::<B>::destroy_all_resources(&mut self.device, &mut self.resource_lists.buffers);
		
		Image::<B>::destroy_all_resources(&mut self.device, &mut self.resource_lists.images);

		Sampler::<B>::destroy_all_resources(&mut self.device, &mut self.resource_lists.samplers);

		self.device.destroy_fence(self.frame_fence);
		self.device.destroy_semaphore(self.frame_semaphore);

		RenderPipeline::<B>::destroy_all_resources(&mut self.device, &mut self.resource_lists.pipelines);

		// for framebuffer in self.resource_lists.framebuffers {
		// 	device.destroy_framebuffer(framebuffer);
		// }
		//self.device.destroy_framebuffer(self.backbuffer);
		// self.destroy_all_resources<?<B>>();

		RenderTarget::<B>::destroy_all_resources(&mut self.device, &mut self.resource_lists.render_targets);

		unimplemented!();
		self.resources_destroyed = true;
	}
}

impl<B> Drop for DeviceController<B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "DeviceController went out of scope without destroy_resources() being called");
	}
}