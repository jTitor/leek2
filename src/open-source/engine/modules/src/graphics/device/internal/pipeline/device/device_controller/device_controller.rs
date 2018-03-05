/*!
 * Provides implementation-independent
 * access to the device's draw calls.
 */
use super::DeviceResourceLists;
use graphics::device::internal::pipeline::{MemoryBuffer, Image, Sampler, RenderPipeline, RenderTarget};

use gfx_hal as hal;
use gfx_hal::{command, pso, memory as m, image as i,
	device as d, format as f};
use gfx_hal::pso::PipelineStage;
use gfx_hal::queue::Submission;

pub struct DeviceController<B: hal::Backend> {
	resource_lists: DeviceResourceLists<B>,

	device: hal::Device<B>,
	command_pool: B::CommandPool,
	//problem here - maybe this has to be external to the struct?
	queue_group: hal::QueueGroup<B, C>,
	main_queue: B::CommandQueue,
	swap_chain: hal::Swapchain<B>,
	backbuffer: hal::Backbuffer<B>,

	viewport: command::Viewport,

	frame_semaphore: B::Semaphore,
	frame_fence: B::Fence,
	frame_wait_timeout_ms: u32,

	frame_can_begin: bool,

	resources_destroyed: bool
}

impl<B> DeviceController<B> where B: hal::Backend {
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

	pub fn upload_to_buffer(&mut self) {
		//TODO_rust: as with submit(), split into actual
		//calls
		let submit = {
			let mut cmd_buffer = self.command_pool.acquire_command_buffer(false);

			let image_barrier = m::Barrier::Image {
				states: (i::Access::empty(), i::ImageLayout::Undefined) ..
						(i::Access::TRANSFER_WRITE, i::ImageLayout::TransferDstOptimal),
				target: &image_logo,
				range: COLOR_RANGE.clone(),
			};
			cmd_buffer.pipeline_barrier(PipelineStage::TOP_OF_PIPE .. PipelineStage::TRANSFER, &[image_barrier]);

			cmd_buffer.copy_buffer_to_image(
				&image_upload_buffer,
				&image_logo,
				i::ImageLayout::TransferDstOptimal,
				&[command::BufferImageCopy {
					buffer_offset: 0,
					buffer_width: row_pitch / (image_stride as u32),
					buffer_height: height as u32,
					image_layers: i::SubresourceLayers {
						aspects: f::AspectFlags::COLOR,
						level: 0,
						layers: 0 .. 1,
					},
					image_offset: command::Offset { x: 0, y: 0, z: 0 },
					image_extent: d::Extent { width, height, depth: 1 },
				}]);

			let image_barrier = m::Barrier::Image {
				states: (i::Access::TRANSFER_WRITE, i::ImageLayout::TransferDstOptimal) ..
						(i::Access::SHADER_READ, i::ImageLayout::ShaderReadOnlyOptimal),
				target: &image_logo,
				range: COLOR_RANGE.clone(),
			};
			cmd_buffer.pipeline_barrier(PipelineStage::TRANSFER .. PipelineStage::FRAGMENT_SHADER, &[image_barrier]);

			cmd_buffer.finish()
		};

		let submission = Submission::new()
			.submit(Some(submit));
		self.main_queue.submit(submission, Some(&mut self.frame_fence));

		self.device.wait_for_fence(&self.frame_fence, self.frame_wait_timeout_ms);
	}

	/**
	 * Performs rendering with the given pipeline.
	 */
	pub fn render_with_pipeline(&mut self, pipeline: &RenderPipeline<B>) {
		self.start_frame();
		//Ask the pipeline for a submission given a command buffer.
		let mut cmd_buffer = self.command_pool.acquire_command_buffer(false);
		let submission = pipeline.submission_with_cmd_buffer(cmd_buffer);
		self.end_frame();
	}

	fn REMOVE_submit(&mut self) {
		//TODO_rust: figure out how to *actually*
		//split this up
		let frame = self.swap_chain.acquire_frame(FrameSync::Semaphore(&mut self.frame_semaphore));

		let submit = {
			let mut cmd_buffer = self.command_pool.acquire_command_buffer(false);

			cmd_buffer.set_viewports(&[viewport.clone()]);
			cmd_buffer.set_scissors(&[viewport.rect]);
			cmd_buffer.bind_graphics_pipeline(&pipeline.as_ref().unwrap());
			cmd_buffer.bind_vertex_buffers(pso::VertexBufferSet(vec![(&vertex_buffer, 0)]));
			cmd_buffer.bind_graphics_descriptor_sets(&pipeline_layout, 0, Some(&desc_set)); //TODO

			{
				let mut encoder = cmd_buffer.begin_render_pass_inline(
					&render_pass,
					&framebuffers[frame.id()],
					viewport.rect,
					&[command::ClearValue::Color(command::ClearColor::Float([0.8, 0.8, 0.8, 1.0]))],
				);
				encoder.draw(0..6, 0..1);
			}

			cmd_buffer.finish()
		};

		let submission = Submission::new()
			.wait_on(&[(&self.frame_semaphore, PipelineStage::BOTTOM_OF_PIPE)])
			.submit(Some(submit));
		self.queue.submit(submission, Some(&mut self.frame_fence));
	}

	pub fn destroy_resources(&mut self) {
		//TODO: Have this be manually dropped
		//to specify order?
		self.device.destroy_command_pool(self.command_pool.downgrade());
		//device.destroy_descriptor_pool(desc_pool);
		//device.destroy_descriptor_set_layout(set_layout);

		//Destroy all the unpacked framebuffers.
		//Destroy all the resources!
		self.destroy_all_resources::<MemoryBuffer<B>>();
		
		self.destroy_all_resources::<Image<B>>();

		self.destroy_all_resources::<Sampler<B>>();

		self.device.destroy_fence(self.frame_fence);
		self.device.destroy_semaphore(self.frame_semaphore);

		self.destroy_all_resources::<RenderPipeline<B>>();

		// for framebuffer in self.resource_lists.framebuffers {
		// 	device.destroy_framebuffer(framebuffer);
		// }
		self.device.destroy_framebuffer(backbuffer);
		// self.destroy_all_resources<?<B>>();

		self.destroy_all_resources::<RenderTarget<B>>();

		unimplemented!();
		self.resources_destroyed = true;
	}
}

impl<B> Drop for DeviceController<B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "DeviceController went out of scope without destroy_resources() being called");
	}
}