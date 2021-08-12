mod modules;
use modules::entity::entity_map;
use modules::vulkan::base::VulkanApp;
use modules::vulkan::instance_creation::VulkanApp1;
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
    let window = VulkanApp1::init_window(&event_loop);
    let mut vulkan_app = VulkanApp1::new();
    vulkan_app.main_loop(event_loop, window);
}
