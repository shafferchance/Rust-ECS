use crate::modules::utility::utility;
use crate::modules::utility::constant::*;
use crate::modules::utility::debug;
use crate::modules::utility::share;

use ash::vk;
use winit::event::{ Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent };
use winit::event_loop::{ EventLoop, ControlFlow };

use std::ptr;

pub const WINDOW_TITLE: &'static str = "15 Hello Triangle";
const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub struct VulkanApp15 {
    window: winit::window::Window,
    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    _physical_device: vk::PhysicalDevice,
    device: ash::Device,

    graphics_queue: vk::Queue,
    present_queue: vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    _swapchain_images: Vec<vk::Image>,
    _swapchain_format: vk::Format,
    _swapchain_extent: vk::Extent2D,
    swapchain_imageviews: Vec<vk::ImageView>,
    swapchain_framebuffers: Vec<vk::Framebuffer>,

    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,

    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    current_frame: usize,
}

impl VulkanApp15 {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> VulkanApp15 {
        let window = utility::init_window(event_loop, WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);

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
            debug::setup_debug_utils(
                &entry, 
                &instance
            );
        let physical_device = 
            share::pick_physical_device(
                &instance, 
                &surface_stuff, 
                &DEVICE_EXTENSIONS
            );
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
                &family_indices,
            );
        let swapchain_imageviews =
            share::create_image_views(
                &device, 
                swapchain_stuff.swapchain_format, 
                &swapchain_stuff.swapchain_images
        );
        let render_pass = share::create_render_pass(&device, swapchain_stuff.swapchain_format);
        let (graphics_pipeline, pipeline_layout) = share::create_graphics_pipeline_v1(
            &device, 
            render_pass, 
            swapchain_stuff.swapchain_extent
        );
        let swapchain_framebuffers = share::create_framebuffers(
            &device, 
            render_pass, 
            &swapchain_imageviews, 
            swapchain_stuff.swapchain_extent
        );
        let command_pool = share::create_command_pool(
            &device, 
            &family_indices
        );
        let command_buffers = share::create_command_buffers(
            &device, 
            command_pool, 
            graphics_pipeline, 
            &swapchain_framebuffers, 
            render_pass, 
            swapchain_stuff.swapchain_extent
        );
        let sync_objects = share::create_sync_objects(
            &device, 
            MAX_FRAMES_IN_FLIGHT
        );
        
        VulkanApp15 {
            window,
            _entry: entry,
            instance,
            surface: surface_stuff.surface,
            surface_loader: surface_stuff.surface_loader,
            debug_utils_loader,
            debug_messenger,

            _physical_device: physical_device,
            device,

            graphics_queue,
            present_queue,

            swapchain_loader: swapchain_stuff.swapchain_loader,
            swapchain: swapchain_stuff.swapchain,
            _swapchain_format: swapchain_stuff.swapchain_format,
            _swapchain_extent: swapchain_stuff.swapchain_extent,
            _swapchain_images: swapchain_stuff.swapchain_images,
            swapchain_imageviews,
            swapchain_framebuffers,

            pipeline_layout,
            render_pass,
            graphics_pipeline,

            command_pool,
            command_buffers,

            image_available_semaphores: sync_objects.image_available_semaphores,
            render_finished_semaphores: sync_objects.render_finished_semaphores,
            in_flight_fences: sync_objects.inflight_fences,
            current_frame: 0,
        }
    }

    fn draw_frame(&mut self) {
        let wait_fences = [self.in_flight_fences[self.current_frame]];

        let (image_index, _is_sub_optimal) = unsafe {
            self.device
                .wait_for_fences(&wait_fences, true, std::u64::MAX)
                .expect("Failed to wait for Fence");

            self.swapchain_loader
                .acquire_next_image(
                    self.swapchain, 
                    std::u64::MAX, 
                    self.image_available_semaphores[self.current_frame], 
                    vk::Fence::null(),    
                )
                .expect("Failed to acquire next image")
        };

        let wait_semaphores = [self.image_available_semaphores[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finished_semaphores[self.current_frame]];

        let submit_infos = [
            vk::SubmitInfo {
                s_type: vk::StructureType::SUBMIT_INFO,
                p_next: ptr::null(),
                wait_semaphore_count: wait_semaphores.len() as u32,
                p_wait_semaphores: wait_semaphores.as_ptr(),
                p_wait_dst_stage_mask: wait_stages.as_ptr(),
                command_buffer_count: 1,
                p_command_buffers: &self.command_buffers[image_index as usize],
                signal_semaphore_count: signal_semaphores.len() as u32,
                p_signal_semaphores: signal_semaphores.as_ptr(),
            }
        ];

        unsafe {
            self.device
                .reset_fences(&wait_fences)
                .expect("Failed to reset Fence");
            
            self.device
                .queue_submit(
                    self.graphics_queue, 
                    &submit_infos, 
                    self.in_flight_fences[self.current_frame]
                )
                .expect("Failed to execute queue submit");
        }

        let swapchains = [self.swapchain];

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: ptr::null_mut(),
        };

        unsafe {
            self.swapchain_loader
                .queue_present(self.present_queue, &present_info)
                .expect("Failed to execute queue present");
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    pub fn main_loop(mut self, event_loop: EventLoop<()>) {
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
                    self.window.request_redraw();
                },
                Event::RedrawRequested(_window_id) => {
                    self.draw_frame();
                },
                Event::LoopDestroyed => {
                    unsafe {
                        self.device.device_wait_idle()
                            .expect("Failed to wait device idle")
                    };
                }
                _ => ()
            }
        })
    }
}

impl Drop for VulkanApp15 {
    fn drop(&mut self) {
        unsafe {
            for i in 0..MAX_FRAMES_IN_FLIGHT {
                self.device
                    .destroy_semaphore(self.image_available_semaphores[i], None);
                self.device
                    .destroy_semaphore(self.render_finished_semaphores[i], None);
                self.device.destroy_fence(self.in_flight_fences[i], None);
            }

            self.device.destroy_command_pool(self.command_pool, None);

            for &framebuffer in self.swapchain_framebuffers.iter() {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device
                .destroy_render_pass(self.render_pass, None);
            
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
