use ash::vk;
use std::ffi::CString;
use std::ptr;

use winit::event::{ Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent };
use winit::event_loop::{ EventLoop, ControlFlow };
use winit::window::Window;

const WINDOW_TITLE: &'static str = "01.Instance Creation";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const APPLICATION_VERSION: u32 = vk::make_version(1, 0, 0);
const ENGINE_VERSION: u32 = vk::make_version(1, 0, 0);
const API_VERSION: u32 = vk::make_version(1, 0, 92);

pub struct VulkanApp1 {
    _entry: ash::Entry,
    instance: ash::Instance
}

impl VulkanApp1 {
    pub fn new() -> VulkanApp1 {
        // init vulkan stuff
        unsafe {
            let entry = ash::Entry::new().unwrap();
            let instance = VulkanApp1::create_instance(&entry);

            VulkanApp1 {
                _entry: entry,
                instance,
            }
        }
    }

    // TODO: Generalize among a new module as this seem like it will remain constant...
    pub fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window")
    }

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        let app_name = CString::new(WINDOW_TITLE).unwrap();
        let engine_name = CString::new("Vulkan Engine").unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: APPLICATION_VERSION,
            p_engine_name: engine_name.as_ptr(),
            engine_version: ENGINE_VERSION,
            api_version: API_VERSION
        };

        let extension_names = super::utility::required_extension_names();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        };

        instance
    }

    fn draw_frame(&mut self) {
        // Drawing stuff coming soon
    }

    pub fn main_loop(&mut self, event_loop: EventLoop<()>, window: Window) {
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
                _ => {}
            }
        })
    }
}

impl Drop for VulkanApp1 {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}