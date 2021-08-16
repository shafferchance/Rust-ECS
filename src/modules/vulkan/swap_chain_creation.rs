use ash::vk;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::collections::HashSet;

use winit::event_loop::{ EventLoop, ControlFlow };
use winit::event::{ Event, ElementState, KeyboardInput, VirtualKeyCode, WindowEvent, };

use crate::modules::utility::constant::*;
use crate::modules::utility::debug;
use crate::modules::utility::structs::*;
use crate::modules::utility::share::create_instance;
use crate::modules::utility::utility;

pub const WINDOW_TITLE: &'static str = "6 Swap Chain Creation";
const DEVICE_EXTENSIONS: DeviceExtension = DeviceExtension {
    names: ["VK_KHR_swapchain"],
};

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>
}

impl QueueFamilyIndices {
    pub fn new() -> QueueFamilyIndices {
        QueueFamilyIndices {
            graphics_family: None,
            present_family: None
        }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

pub struct VulkanApp6 {
    _entry: ash::Entry,
    _physical_device: vk::PhysicalDevice,
    _graphics_queue: vk::Queue,
    _present_queue: vk::Queue,
    _swapchain_images: Vec<vk::Image>,
    _swapchain_format: vk::Format,
    _swapchain_extent: vk::Extent2D,
    
    debug_messenger: vk::DebugUtilsMessengerEXT,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    device: ash::Device,
    instance: ash::Instance,
    surface: vk::SurfaceKHR,
    surface_loader: ash::extensions::khr::Surface,
    swapchain: vk::SwapchainKHR,
    swapchain_loader: ash::extensions::khr::Swapchain,   
}

impl VulkanApp6 {
    pub fn new(window: &winit::window::Window) -> VulkanApp6 {
        let entry = unsafe {
            ash::Entry::new().unwrap()
        };
        let instance = create_instance(
            &entry, 
            WINDOW_TITLE, 
            VALIDATION.is_enable, 
            &VALIDATION.required_validation_layers.to_vec(),
        );
        let surface_stuff = VulkanApp6::create_surface(
            &entry,
            &instance,
            &window
        );
        let (debug_utils_loader, debug_messenger) =
            debug::setup_debug_utils(&entry, &instance);
        let physical_device = VulkanApp6::pick_physical_device(&instance, &surface_stuff);
        let (device, family_indices) = VulkanApp6::create_logical_device(
            &instance,
            physical_device,
            &VALIDATION,
            &surface_stuff
        );
        let graphics_queue =
            unsafe { device.get_device_queue(family_indices.graphics_family.unwrap(), 0) };
        let present_queue =
            unsafe { device.get_device_queue(family_indices.present_family.unwrap(), 0) };
        let swapchain_stuff = VulkanApp6::create_swapchain(
            &instance,
            &device,
            physical_device,
            &surface_stuff,
            &family_indices
        );

        VulkanApp6 {
            _entry: entry,
            _physical_device: physical_device,
            _graphics_queue: graphics_queue,
            _present_queue: present_queue,
            _swapchain_extent: swapchain_stuff.swapchain_extent,
            _swapchain_format: swapchain_stuff.swapchain_format,
            _swapchain_images: swapchain_stuff.swapchain_images,
            surface: surface_stuff.surface,
            surface_loader: surface_stuff.surface_loader,
            swapchain: swapchain_stuff.swapchain,
            swapchain_loader: swapchain_stuff.swapchain_loader,
            debug_messenger,
            debug_utils_loader,
            device,
            instance,
        }
    }

    fn create_surface(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &winit::window::Window
    ) -> SurfaceStuff {
        let surface = unsafe {
            utility::create_surface(entry, instance, window)
                        .expect("Failed to create surface!")
        };
        
        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

        SurfaceStuff {
            surface,
            surface_loader,
            screen_width: WINDOW_WIDTH,
            screen_height: WINDOW_HEIGHT
        }
    }

    fn pick_physical_device(
        instance: &ash::Instance,
        surface_stuff: &SurfaceStuff
    ) -> vk::PhysicalDevice {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate Physical Devices!")
        };

        let result = physical_devices.iter().find(|physical_device| {
            VulkanApp6::is_physical_device_suitable(
                instance,
                **physical_device,
                surface_stuff
            )
        });

        match result {
            Some(p_physical_device) => *p_physical_device,
            None => panic!("Failed to find a suitable GPU!")
        }
    }

    fn is_physical_device_suitable(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_stuff: &SurfaceStuff
    ) -> bool {
        let _device_features = unsafe {
            instance.get_physical_device_features(physical_device)
        };

        let indices = VulkanApp6::find_queue_family(instance, physical_device, surface_stuff);

        let is_queue_family_supported = indices.is_complete();
        let is_device_extension_supported =
            VulkanApp6::check_device_extension_support(instance, physical_device);
        let is_swapchain_supported = if is_device_extension_supported {
            let swapchain_support = VulkanApp6::query_swapchain_support(physical_device, surface_stuff);
            !swapchain_support.formats.is_empty() && !swapchain_support.present_modes.is_empty()
        } else {
            false
        };

        is_queue_family_supported
        && is_device_extension_supported
        && is_swapchain_supported
    }

    fn create_logical_device(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        validation: &debug::ValidationInfo,
        surface_stuff: &SurfaceStuff
    ) -> (ash::Device, QueueFamilyIndices) {
        let indices = VulkanApp6::find_queue_family(instance, physical_device, surface_stuff);

        let mut unique_queue_families = HashSet::new();
        unique_queue_families.insert(indices.graphics_family.unwrap());
        unique_queue_families.insert(indices.present_family.unwrap());

        let queue_priorities = [1.0_f32];
        let mut queue_create_infos = vec![];
        for &queue_family in unique_queue_families.iter() {
            let queue_create_info = vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: queue_family,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: queue_priorities.len() as u32,
            };
            queue_create_infos.push(queue_create_info);
        }

        let physical_device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let required_validation_layer_raw_names: Vec<CString> = validation
            .required_validation_layers
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();
        let enable_layer_names: Vec<*const c_char> = required_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let enable_extension_names = [
            // This enables the swapchain extension
            ash::extensions::khr::Swapchain::name().as_ptr()
        ];

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: if validation.is_enable {
                enable_layer_names.len()
            } else {
                0
            } as u32,
            pp_enabled_layer_names: if validation.is_enable {
                enable_layer_names.as_ptr()
            } else {
                ptr::null()
            },
            enabled_extension_count: enable_extension_names.len() as u32,
            pp_enabled_extension_names: enable_extension_names.as_ptr(),
            p_enabled_features: &physical_device_features
        };

        let device: ash::Device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device.")
        };

        (device, indices)
    }
    
    fn find_queue_family(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_stuff: &SurfaceStuff
    ) -> QueueFamilyIndices {
        let queue_families =  unsafe {
            instance.get_physical_device_queue_family_properties(physical_device)
        };
        let mut queue_family_indices = QueueFamilyIndices::new();

        let mut index = 0;
        for queue_family in queue_families.iter() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                queue_family_indices.graphics_family = Some(index);
            }

            let is_present_support = unsafe {
                surface_stuff
                    .surface_loader
                    .get_physical_device_surface_support(physical_device, index as u32, surface_stuff.surface)
                    .expect("Couldn't find supported Physical Device!")
            };

            if queue_family.queue_count > 0 && is_present_support {
                queue_family_indices.present_family = Some(index);
            }

            if queue_family_indices.is_complete() {
                break;
            }

            index += 1;
        }

        queue_family_indices
    }

    fn check_device_extension_support(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice
    ) -> bool {
        let available_extensions = unsafe {
            instance
                .enumerate_device_extension_properties(physical_device)
                .expect("Failed to get device extension properties")
        };

        let mut available_extension_names = vec![];

        println!("\tAvailable Device Extensions: ");
        for extension in available_extensions.iter() {
            let extension_name = utility::vk_to_string(&extension.extension_name);
            println!(
                "\t\tName: {}, Version: {}",
                extension_name, extension.spec_version
            );

            available_extension_names.push(extension_name);
        }

        let mut required_extensions = HashSet::new();
        for extension in DEVICE_EXTENSIONS.names.iter() {
            required_extensions.insert(extension.to_string());
        }

        for extension_name in available_extension_names.iter() {
            required_extensions.remove(extension_name);
        }

        return required_extensions.is_empty();
    }

    fn query_swapchain_support(
        physical_device: vk::PhysicalDevice,
        surface_stuff: &SurfaceStuff
    ) -> SwapChainSupportDetail {
        unsafe {
            let capabilities = surface_stuff
                .surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface_stuff.surface)
                .expect("Failed to query for surface capabilities");
            let formats = surface_stuff
                .surface_loader
                .get_physical_device_surface_formats(physical_device, surface_stuff.surface)
                .expect("Failed to query for surface formats");
            let present_modes = surface_stuff
                .surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface_stuff.surface)
                .expect("Failed to query for surface present mode");

            SwapChainSupportDetail {
                capabilities,
                formats,
                present_modes
            }
        }
    }

    fn create_swapchain(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: vk::PhysicalDevice,
        surface_stuff: &SurfaceStuff,
        queue_family: &QueueFamilyIndices
    ) -> SwapChainStuff {
        let swapchain_support = VulkanApp6::query_swapchain_support(physical_device, surface_stuff);

        let surface_format = VulkanApp6::choose_swapchain_format(&swapchain_support.formats);
        let present_mode =
            VulkanApp6::choose_swapchain_present_mode(&swapchain_support.present_modes);
        let extent = VulkanApp6::choose_swapchain_extent(&swapchain_support.capabilities);

        // Basically initializing with the smallest value to take advantage of min function on primitive u32
        let image_count = swapchain_support.capabilities.min_image_count + 1;
        let image_count = if swapchain_support.capabilities.max_image_count > 0 {
            image_count.min(swapchain_support.capabilities.max_image_count)
        } else {
            image_count
        };

        let (image_sharing_mode, queue_family_index_count, queue_family_indices) =
            if queue_family.graphics_family != queue_family.present_family {
                (
                    vk::SharingMode::EXCLUSIVE,
                    2,
                    vec![
                        queue_family.graphics_family.unwrap(),
                        queue_family.present_family.unwrap(),
                    ],
                )
            } else {
                (vk::SharingMode::EXCLUSIVE, 0, vec![])
            };
        
        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: surface_stuff.surface,
            min_image_count: image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count,
            pre_transform: swapchain_support.capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
            image_array_layers: 1,
        };

        let swapchain_loader = ash::extensions::khr::Swapchain::new(instance, device);
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Familed to create Swapchain!")
        };
        let swapchain_images = unsafe {
            swapchain_loader
                .get_swapchain_images(swapchain)
                .expect("Failed to get Swapchain Images.")
        };

        SwapChainStuff {
            swapchain_format: surface_format.format,
            swapchain_extent: extent,
            swapchain_loader,
            swapchain,
            swapchain_images,
        }
    }

    fn choose_swapchain_format(
        available_formats: &Vec<vk::SurfaceFormatKHR>,
    ) -> vk::SurfaceFormatKHR {
        // check if list conatins most widely used R8G8B8A8 format with non-linear color space
        for available_format in available_formats {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return available_format.clone();
            }
        }

        return available_formats.first().unwrap().clone();
    }

    fn choose_swapchain_present_mode(
        available_present_modes: &Vec<vk::PresentModeKHR>,
    ) -> vk::PresentModeKHR {
        for &available_present_mode in available_present_modes.iter() {
            if available_present_mode == vk::PresentModeKHR::MAILBOX {
                return available_present_mode;
            }
        }

        vk::PresentModeKHR::FIFO
    }

    fn choose_swapchain_extent(capabilities: &vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::max_value() {
            capabilities.current_extent
        } else {
            vk::Extent2D {
                width: u32::clamp(
                    WINDOW_WIDTH,
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: u32::clamp(
                    WINDOW_HEIGHT,
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height
                )
            }
        }
    }

    fn draw_frame(&mut self) {
        // sdafkasdfasdf
    }

    pub fn main_loop(mut self, event_loop: EventLoop<()>, window: winit::window::Window) {
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                KeyboardInput { virtual_keycode, state, .. } => { 
                                    match (virtual_keycode, state) {
                                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            *control_flow = ControlFlow::Exit
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                },
                Event::RedrawRequested(_window_id) => {
                    self.draw_frame();
                },
                _ => ()
            }
        })
    }
}
