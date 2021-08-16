use ash::vk;

use winit::event::{ Event, ElementState, KeyboardInput, WindowEvent, VirtualKeyCode };
use winit::event_loop::{ EventLoop, ControlFlow };

use std::ptr;

use crate::modules::utility::constant::*;
use crate::modules::utility::share;
use crate::modules::utility::utility;
use crate::modules::utility::debug;

pub const WINDOW_TITLE: &'static str = "7 Image Version";

pub struct VulkanApp7 {
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

impl VulkanApp7 {
    pub fn new(window: &winit::window::Window) -> VulkanApp7 {
        let entry = unsafe {
            ash::Entry::new().expect("Failed to create Entry!")
        };
        let instance = share::create_instance(
            &entry, 
            WINDOW_TITLE, 
            VALIDATION.is_enable,
            &VALIDATION.required_validation_layers.to_vec()
        );

        let surface_stuff = share::create_surface(
            &entry, 
            &instance,
            window,
            WINDOW_WIDTH,
            WINDOW_HEIGHT
        );
        let (debug_utils_loader, debug_messenger) =
            debug::setup_debug_utils(&entry, &instance);
        let physical_device = 
            share::pick_physical_device(&instance, &surface_stuff);
        let (device, family_indices) = 
            share::create_logical_device(
                &instance, 
                physical_device, 
                &VALIDATION, 
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
                &surface_stuff, 
                &family_indices
        );
        let swapchain_imageviews = VulkanApp7::create_image_views(
            &device,
            swapchain_stuff.swapchain_format,
            &swapchain_stuff.swapchain_images,
        );

        VulkanApp7 {
            _entry: entry,
            _graphics_queue: graphics_queue,
            _physical_device: physical_device,
            _present_queue: present_queue,
            _swapchain_extent: swapchain_stuff.swapchain_extent,
            _swapchain_format: swapchain_stuff.swapchain_format,
            _swapchain_images: swapchain_stuff.swapchain_images,
            surface: surface_stuff.surface,
            surface_loader: surface_stuff.surface_loader,
            swapchain: swapchain_stuff.swapchain,
            swapchain_loader: swapchain_stuff.swapchain_loader,
            swapchain_imageviews,
            debug_messenger,
            debug_utils_loader,
            device,
            instance,
        }
    }

    fn create_image_views(
        device: &ash::Device,
        surface_format: vk::Format,
        images: &Vec<vk::Image>,
    ) -> Vec<vk::ImageView> {
        let mut swapchain_imageviews = vec![];

        for &image in images.iter() {
            let imageview_create_info = vk::ImageViewCreateInfo {
                s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::ImageViewCreateFlags::empty(),
                view_type: vk::ImageViewType::TYPE_2D,
                format: surface_format,
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY
                },
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                image,
            };

            let imageview = unsafe {
                device
                    .create_image_view(&imageview_create_info, None)
                    .expect("Failed to create Image View")
            };
            swapchain_imageviews.push(imageview);
        }

        swapchain_imageviews
    }

    fn draw_frame(&mut self) {
        //adsfasdf
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

impl Drop for VulkanApp7 {
    fn drop(&mut self) {
        unsafe {
            for &imageview in self.swapchain_imageviews.iter() {
                self.device.destroy_image_view(imageview, None);
            }

            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
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
