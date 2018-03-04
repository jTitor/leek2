/*!
 * okay you are going to have to come up
 * with a more descriptive name than "device"
 */

mod device_info;
pub use self::device_info::DeviceInfo;

mod device_controller;
pub use self::device_controller::DeviceController;

mod device_controller_builder;
pub use self::device_controller_builder::DeviceControllerBuilder;

mod device_resource;
pub use self::device_resource::DeviceResource;

mod viewport;
pub use self::viewport::Viewport;