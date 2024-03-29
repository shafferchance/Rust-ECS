mod modules;
use modules::entity::entity_map;
use modules::utility::utility::init_window;
// use modules::vulkan::base::VulkanApp;
use modules::vulkan::instance_creation::VulkanApp1;
use modules::vulkan::validation_layers::VulkanApp2;
// Import WINDOW_TITLE from files
use modules::vulkan::physical_device::{ VulkanApp3, WINDOW_TITLE as WINDOW_TITLE_3 };
use modules::vulkan::logical_device::{ VulkanApp4, WINDOW_TITLE as WINDOW_TITLE_4 };
use modules::vulkan::window_surface::{ VulkanApp5, WINDOW_TITLE as WINDOW_TITLE_5 };
use modules::vulkan::swap_chain_creation::{ VulkanApp6, WINDOW_TITLE as WINDOW_TITLE_6 };
use modules::vulkan::image_view::{ VulkanApp7, WINDOW_TITLE as WINDOW_TITLE_7 };
use modules::vulkan::shader_modules::{ VulkanApp9, WINDOW_TITLE as WINDOW_TITLE_9 };
use modules::vulkan::hello_triangle::VulkanApp15;

use modules::utility::constant::*;
use winit::event_loop::EventLoop;

fn main() {
    let mut entity_map = entity_map::EntityMap::<u64>::new();

    let entity_1 = entity_map.insert(1);
    let entity_2 = entity_map.insert(2);
    let entity_3 = entity_map.insert(3);
    let entity_4 = entity_map.insert(4);

    entity_1.print();
    entity_2.print();
    entity_3.print();
    entity_4.print();

    // Base Example
    let event_loop = EventLoop::new(); // The same across examples
    // let _window = VulkanApp::init_window(&event_loop);

    // VulkanApp::main_loop(event_loop);

    // Example 1
    // let window = VulkanApp1::init_window(&event_loop);
    // let mut vulkan_app = VulkanApp1::new();
    // vulkan_app.main_loop(event_loop, window);

    // Example 2
    // let window = VulkanApp2::init_window(&event_loop);
    // let vulkan_app = VulkanApp2::new();
    // vulkan_app.main_loop(event_loop, window);

    // Example 3
    // let window = init_window(&event_loop, WINDOW_TITLE_3, WINDOW_WIDTH, WINDOW_HEIGHT);
    // let vulkan_app = VulkanApp3::new();
    // vulkan_app.main_loop(event_loop, window);

    // Example 4
    // let window = init_window(&event_loop, WINDOW_TITLE_4, WINDOW_WIDTH, WINDOW_HEIGHT);
    // let vulkan_app = VulkanApp4::new();
    // vulkan_app.main_loop(event_loop, window);

    // Example 5
    // let window = init_window(&event_loop, WINDOW_TITLE_5, WINDOW_WIDTH, WINDOW_HEIGHT);
    // let vulkan_app = VulkanApp5::new(&window);
    // vulkan_app.main_loop(event_loop, window);

    // Example 6
    // let window = init_window(&event_loop, WINDOW_TITLE_6, WINDOW_WIDTH, WINDOW_HEIGHT);
    // let vulkan_app = VulkanApp6::new(&window);
    // vulkan_app.main_loop(event_loop, window);

    // Example 7
    // let window = init_window(&event_loop, WINDOW_TITLE_7, WINDOW_WIDTH, WINDOW_HEIGHT);
    // let vulkan_app = VulkanApp7::new(&window);
    // vulkan_app.main_loop(event_loop, window);

    // Example 9
    // let window = init_window(&event_loop, WINDOW_TITLE_9, WINDOW_WIDTH, WINDOW_HEIGHT);
    // let vulkan_app = VulkanApp9::new(&window);
    // vulkan_app.main_loop(event_loop, window);

    // Example 15
    let vulkan_app = VulkanApp15::new(&event_loop);
    vulkan_app.main_loop(event_loop);
}
