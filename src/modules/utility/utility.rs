use ash::vk;
use std::ffi::CStr;
use std::os::raw::c_char;

use winit::event_loop::EventLoop;

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

// #[cfg(target_os = "macos")]
// use cocoa::appkit::{NSView, NSWindow};
// #[cfg(target_os = "macos")]
// use cocoa::base::id as cocoa_id;
// #[cfg(target_os = "macos")]
// use metal::CoreAnimationLayer;
// #[cfg(target_os = "macos")]
// use objc::runtime::YES;

// required extensions ----------------------------------------------------------------
#[cfg(target_os = "windows")]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

// #[cfg(target_os = "macos")]
// pub fn required_extension_names() -> Vec<*const i8> {
//     vec![
//         Surface::name().as_ptr(),
//         MacOSSurface::name().as_ptr(),
//         DebugUtils::name().as_ptr(),
//     ]
// }

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

// create surface -----------------------------------------------------------------------
#[cfg(target_os = "windows")]
pub unsafe fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window: &winit::window::Window
) -> Result<vk::SurfaceKHR, vk::Result> {
    use std::os::raw::c_void;
    use std::ptr;
    use winapi::shared::windef::HWND;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winit::platform::windows::WindowExtWindows;

    let hwnd = window.hwnd() as HWND;
    let hinstance = GetModuleHandleW(ptr::null()) as *const c_void;
    let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
        s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags: Default::default(),
        hwnd: hwnd as *const c_void,
        hinstance,
    };
    let win32_surface_loader = Win32Surface::new(entry, instance);
    win32_surface_loader.create_win32_surface(&win32_create_info, None)
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub unsafe fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window: &winit::window::Window
) -> Result<vk::SurfaceKHR, vk::Result> {
    use std::ptr;
    use winit::platform::unix::WindowExtUnix;

    let x11_display = window.xlib_display().unwrap();
    let x11_window = window.xlib_window().unwrap();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
        s_type: vk::StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags: Default::default(),
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display
    };
    let xlib_surface_loader = XlibSurface::new(entry, instance);
    xlib_surface_loader.create_xlib_surface(&x11_create_info, None)
}

// #[cfg(target_os = "macos")]
// pub unsafe fn create_surface(
//     entry: &ash::Entry,
//     instance: &ash::Instance,
//     window: &winit::window::Window
// ) -> Result<vk::SurfaceKHR, vk::Result> {
//     use std::mem;
//     use std::os::raw::c_void;
//     use std::ptr;
//     use winit::platform::macos::WindowExtMacOS;

//     let wnd: cocoa_id = mem::transmute(window.ns_window());
//     let layer = CoreAnimationLayer::new();

//     layer.set_edge_antialiasing_mask(0);
//     layer.set_presents_with_transaction(false);
//     layer.remove_all_animations();

//     let view = wnd.contentView();
    
//     layer.set_contents_scale(view.backingScaleFactor());
//     view.setLayer(mem::transmute(layer.as_ref()));
//     view.setWantsLayer(YES);

//     let create_info = vk::MacOSSurfaceCreateInfoMVK {
//         s_type: vk::StructureType::MACOS_SURFACE_CREATE_INFO_M,
//         p_next: ptr::null(),
//         flags: Default::default(),
//         p_view: window.ns_view() as *const c_void,
//     };

//     let macos_surface_loader = MacOSSurface::new(entry, instance);
//     macos_surface_loader.create_mac_os_surface(&create_info, None)
// }

pub fn vk_to_string(raw_string_array: &[c_char]) -> String {
    // Unsafe is required as we're deferencing to upcast the pointer (my best guess)
    let raw_string = unsafe {
        let ptr = raw_string_array.as_ptr();
        CStr::from_ptr(ptr)
    };

    raw_string
        .to_str()
        .expect("Failed to convert Vulkan raw string")
        .to_owned()
}

pub fn init_window(
    event_loop: &EventLoop<()>,
    title: &str,
    width: u32,
    height: u32
) -> winit::window::Window {
    winit::window::WindowBuilder::new()
        .with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(event_loop)
        .expect("Failed to create window")
}
