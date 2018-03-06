/*!
 * Abstracts rendering commands to the backend
 * implementation.
 * 
 * Right now the command submission interface
 * hasn't been designed, so the demo app
 * in ```bin/graphics_pipeline_demo.rs```
 * uses the gfx_hal submission API when
 * submitting its draw request.
 */

mod device_controller;
pub use self::device_controller::DeviceController;

mod resource_lists;
pub use self::resource_lists::DeviceResourceLists;