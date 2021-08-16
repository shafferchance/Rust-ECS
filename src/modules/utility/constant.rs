use ash::vk;
use std::os::raw::c_char;

use crate::modules::utility::debug::ValidationInfo;
use crate::modules::utility::structs::*;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const APPLICATION_VERSION: u32 = vk::make_version(1, 0, 0);
pub const ENGINE_VERSION: u32 = vk::make_version(1, 0, 0);
pub const API_VERSION: u32 = vk::make_version(1, 0, 92);

pub const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
};

pub const DEVICE_EXTENSIONS: DeviceExtension = DeviceExtension {
    names: ["VK_KHR_swapchain"]
};

impl DeviceExtension {
    pub fn get_extensions_raw_names(&self) -> [*const c_char; 1] {
        [
            ash::extensions::khr::Swapchain::name().as_ptr()
        ]
    }
}
