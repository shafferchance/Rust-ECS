use ash::vk;
use winit::event::{ Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent };
use winit::event_loop::{ EventLoop, ControlFlow };

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use crate::modules::utility::constant::*;
use crate::modules::utility::debug;
use crate::modules::utility::share;
use crate::modules::utility::utility;

pub const WINDOW_TITLE: &'static str = "5 Window Surface";

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn new() -> QueueFamilyIndices {
        QueueFamilyIndices {
            graphics_family: None,
            present_family: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

struct SurfaceStuff {
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceFormatKHR
}

pub struct VulkanApp5 {
    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,
    _physical_device: vk::PhysicalDevice,
    device: ash::Device,
    _graphics_queue: vk::Queue,
    _present_queue: vk::Queue
}

impl VulkanApp5 {
    pub fn new(window: &winit::window::Window) -> VulkanApp5 {
        let entry = unsafe { ash::Entry::new().unwrap() };
        let instance = share::create_instance(&entry, WINDOW_TITLE, VALIDATION.is_enable, &VALIDATION.required_validation_layers.to_vec());
        let (debug_utils_loader, debug_messenger) = debug::setup_debug_utils(&entry, &instance);
        let surface_stuff = VulkanApp5::create_surface(&entry, &instance, &window);
        let physical_device = VulkanApp5::pick_physical_device(&instance, &surface_stuff);
        let (device, family_indices) = VulkanApp5::create_logical_device(
            &instance,
            physical_device,
            &VALIDATION,
            &surface_stuff
        );
        let graphics_queue = unsafe { device.get_device_queue(family_indices.graphics_family.unwrap(), 0) };
        let present_queue = unsafe { device.get_device_queue(family_indices.present_family.unwrap(), 0) };

        VulkanApp5 {
            _entry: entry,
            _graphics_queue: graphics_queue,
            _physical_device: physical_device,
            _present_queue: present_queue,
            surface: surface_stuff.surface,
            surface_loader: surface_stuff.surface_loader,
            debug_messenger,
            debug_utils_loader,
            device
        }
    }

    fn create_surface(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &winit:window::Window
    ) -> SurfaceStuff {
        let surface = unsafe {
            utility::create_surface(entry, instance, window)
                .expect("Failed to create surface")
        };
        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

        SurfaceStuff { 
            surface_loader,
            surface
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
        }

        let result = physical_devices.iter().find(|physical_device| {
            VulkanApp5::is_physical_device_suitable(instance, **physical_device, surface_stuff)
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
        let _device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let _device_features = unsafe { instance.get_physical_device_features(physical_device) };
        let indices = VulkanApp5::find_queue_family(instance, physical_device, surface_stuff);

        return indices.is_complete();
    }

    fn create_logical_device(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        validation: &debug::ValidationInfo,
        surface_stuff: &SurfaceStuff
    ) -> (ash::Device, QueueFamilyIndices) {
        let indices = VulkanApp5::find_queue_family(instance, physical_device, surface_stuff);

        use std::collections::HashSet;
        let mut unique_queue_families = HashSet::new();
        unique_queue_families.insert(indices.graphics_family.unwrap());
        unique_queue_families.insert(indices.present_family.unwrap());

        let queue_priorities = [1.0_f32];
        let mut queue_create_infos = vec![];
        for &queue_family in unique_queue_families.iter() {
            let queue_create_info = vk::DEVICE_QUEUE_CREATE_INFO {
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: queue_family,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: queue_priorities.len() as u32,
            };
            queue_create_infos.push(queue_create_info)
        }
        
        let physical_device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let required_validation_layer_raw_names: Vec<CString> = validation
            .required_validation_layers
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();
        let enabled_layer_names: Vec<*const c_char> required_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: if validation.is_enable {
                enabled_layer_names.lens()
            } else {
                0
            } as u32;
            pp_enabled_layer_names: if validation.is_enable {
                enabled_layer_names.as_ptr()
            } else {
                ptr::null()
            },
            enabled_extension_count: 0,
            pp_enabled_extension_names: ptr::null(),
            p_enabled_features: &physical_device_features
        };

        let device: ash::Device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device!")
        };

        (device, indices)
    }

    fn find_queue_family(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_stuff: &SurfaceStuff
    ) -> QueueFamilyIndices {
        let queue_families = unsafe { 
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

    fn draw_frame(&mut self) {
        // adsfasdf
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
                                        _ => {},
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

impl Drop for VulkanApp5 {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            if VALIDATION.is_enable {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
