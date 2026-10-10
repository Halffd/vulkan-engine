#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use pretty_env_logger;

// Winit imports
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

// Vulkan imports
use vulkanalia::loader::{LIBRARY, LibloadingLoader};
use vulkanalia::vk::{self, HasBuilder, InstanceV1_0};
use vulkanalia::{Entry, Instance};
use vulkanalia::Version;

mod fm;
use fm::FM;
mod config;
use config::{Files, Paths, find_config_files, load_config, merge_config_files};
use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;

// Note: This trait was called `ExtDebugUtilsExtension` in versions of `vulkanalia` prior to `v0.31.0`.
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;
H
const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

fn main() -> Result<()> {
    pretty_env_logger::init();

    let fm = FM::new();

    // Asset paths
    let paths = Paths::default();
    let files = Files::default();

    for path in [paths.sprite, paths.texture, paths.shader, paths.mesh] {
        fm.ls_dir(path);
    }

    // Config files
    if let Some(config_file) = find_config_files(&files) {
        println!("Config files: {:?}", config_file);
        // merge configs
        let config = merge_config_files(&config_file)?;
        println!("Config: {}", config);
        // parse config (simplified for now)
        println!("Config: {}", config);
    }

    // Window configuration
    let window_config = load_config()?;

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title(&window_config.title)
        .with_inner_size(LogicalSize::new(window_config.width, window_config.height))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? };
    event_loop.run(move |event, elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() => {
                    unsafe { app.render(&window) }.unwrap()
                }
                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe {
                        app.destroy();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    })?;

    Ok(())
}

/// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
}
impl App {
    /// Creates our Vulkan app.
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = unsafe { LibloadingLoader::new(LIBRARY)? };
        let entry = unsafe {
            Entry::new(loader).map_err(|e| anyhow::anyhow!("Failed to create entry: {}", e))?
        };
        let instance = unsafe { create_instance(window, &entry)? };
        Ok(Self { entry, instance })
    }

    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    unsafe fn destroy(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}
unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Engine")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));
    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216.
    let flags = if 
        cfg!(target_os = "macos") && 
        entry.version()? >= PORTABILITY_MACOS_VERSION
    {
        info!("Enabling extensions for macOS portability.");
        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensions)
        .flags(flags);
    Ok(unsafe { entry.create_instance(&info, None)? })
}
