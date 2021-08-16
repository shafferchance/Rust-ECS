use ash::vk;
use winit::event::{ Event, ElementState, KeyboardInput, VirtualKeyCode, WindowEvent };
use winit::event_loop::{ EventLoop, ControlFlow };

use std::ffi::CString;
use std::ptr;
use std::env;

use crate::modules::utility::constant::*;
use crate::modules::utility::share;
use crate::modules::utility::debug;
use crate::modules::utility::utility;

use std::path::Path;

pub const WINDOW_TITLE: &'static str = "9 Shader Modules";

pub struct VulkanApp9 {
    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    _physical_device: vk::PhysicalDevice,
    device: ash::Device,

    _graphics_queue: vk::Queue,
    _present_queue: vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    _swapchain_images: Vec<vk::Image>,
    _swapchain_format: vk::Format,
    _swapchain_extent: vk::Extent2D,
    swapchain_imageviews: Vec<vk::ImageView>,
}

impl VulkanApp9 {
    pub fn new(window: &winit::window::Window) -> VulkanApp9 {
        let entry = unsafe { ash::Entry::new().expect("Failed to create Entry") };
        let instance = share::create_instance(
            &entry, 
            WINDOW_TITLE, 
            VALIDATION.is_enable, 
            &VALIDATION.required_validation_layers.to_vec()
        );
        let surface_stuff =
            share::create_surface(
                &entry, 
                &instance, 
                &window, 
                WINDOW_WIDTH, 
                WINDOW_HEIGHT
            );
        let (debug_utils_loader, debug_messenger) =
            debug::setup_debug_utils(&entry, &instance);
        let physical_device = 
            share::pick_physical_device(&instance, &surface_stuff, &DEVICE_EXTENSIONS);
        let (device, family_indices) = 
            share::create_logical_device(
                &instance, 
                physical_device, 
                &VALIDATION, 
                &DEVICE_EXTENSIONS,
                &surface_stuff
            );
        let graphics_queue = 
            unsafe { device.get_device_queue(family_indices.graphics_family.unwrap(), 0) };
        let present_queue =
            unsafe { device.get_device_queue(family_indices.present_family.unwrap(), 0) };
        let swapchain_stuff =
            share::create_swapchain(
                &instance, 
                &device, 
                physical_device,
                &window,
                &surface_stuff, 
                &family_indices
            );
        let swapchain_imageviews = 
            share::create_image_views(
                &device, 
                swapchain_stuff.swapchain_format, 
                &swapchain_stuff.swapchain_images
            );
        let _graphics_pipeline = VulkanApp9::create_graphics_pipeline(&device);

        VulkanApp9 {
            _entry: entry,
            instance,
            surface: surface_stuff.surface,
            surface_loader: surface_stuff.surface_loader,
            debug_utils_loader,
            debug_messenger,

            _physical_device: physical_device,
            device,

            _graphics_queue: graphics_queue,
            _present_queue: present_queue,

            swapchain_loader: swapchain_stuff.swapchain_loader,
            swapchain: swapchain_stuff.swapchain,
            _swapchain_format: swapchain_stuff.swapchain_format,
            _swapchain_images: swapchain_stuff.swapchain_images,
            _swapchain_extent: swapchain_stuff.swapchain_extent,
            swapchain_imageviews,
        }
    }

    fn create_graphics_pipeline(device: &ash::Device) {
        let vert_shader_code =
            VulkanApp9::read_shader_code(Path::new("shaders/spv/9-shader-base.vert.spv"));
        let frag_shader_code =
            VulkanApp9::read_shader_code(Path::new("shaders/spv/9-shader-base.frag.spv"));

        let vert_shader_module = VulkanApp9::create_shader_module(device, vert_shader_code);
        let frag_shader_module = VulkanApp9::create_shader_module(device, frag_shader_code);

        let main_function_name = CString::new("main").unwrap();

        let _shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                // Vertex Shader
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: vert_shader_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::VERTEX,
            },
            vk::PipelineShaderStageCreateInfo {
                // Fragment Shader
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: frag_shader_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::FRAGMENT
            },
        ];

        unsafe {
            device.destroy_shader_module(vert_shader_module, None);
            device.destroy_shader_module(frag_shader_module, None);
        }
    }

    fn create_shader_module(device: &ash::Device, code: Vec<u8>) -> vk::ShaderModule {
        let shader_module_create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: code.len(),
            p_code: code.as_ptr() as *const u32,
        };

        unsafe {
            device
                .create_shader_module(&shader_module_create_info, None)
                .expect("Failed to create Shader Module")
        }
    }

    fn read_shader_code(shader_path: &Path) -> Vec<u8> {
        use std::fs::File;
        use std::io::Read;

        let spv_file = File::open(shader_path)
            .expect(&format!("Failed to find spv file at {:?}", shader_path));
        let bytes_code: Vec<u8> = spv_file.bytes().filter_map(|byte| byte.ok()).collect();

        bytes_code
    }

    fn draw_frame(&mut self) {
        // fasdfadsf
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
