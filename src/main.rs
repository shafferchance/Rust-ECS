mod modules;
use modules::entity::entity_map;
use modules::vulkan::base::VulkanApp;
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

    let event_loop = EventLoop::new();
    let _window = VulkanApp::init_window(&event_loop);

    VulkanApp::main_loop(event_loop);
}
