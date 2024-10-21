mod camera;
mod camera_controller;
mod instance;
pub mod object;
mod state;

use object::Object;
use state::State;
use std::{
    sync::{Arc, Mutex},
    thread::{self},
};
use winit::{
    event::*,
    event_loop::EventLoopBuilder,
    keyboard::{KeyCode, PhysicalKey},
    platform::wayland::EventLoopBuilderExtWayland,
    window::WindowBuilder,
};

pub fn run() -> Arc<Mutex<Vec<Object>>> {
    env_logger::init();
    let ext_objects = Arc::new(Mutex::new(vec![]));
    let int_objects = Arc::clone(&ext_objects);

    thread::spawn(|| {
        let event_loop = EventLoopBuilder::new()
            .with_any_thread(true)
            .build()
            .unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let mut state = pollster::block_on(State::new(&window, int_objects));
        let mut surface_configured = false;
        event_loop
            .run(move |event, control_flow| {
                if let Event::WindowEvent {
                    ref event,
                    window_id: _,
                } = event
                {
                    state.input(event);
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            log::info!("physical_size: {physical_size:?}");
                            surface_configured = true;
                            state.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            // This tells winit that we want another frame after this one
                            state.window().request_redraw();

                            if !surface_configured {
                                return;
                            }

                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if it's lost or outdated
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    state.resize(state.size())
                                }
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    log::error!("OutOfMemory");
                                    control_flow.exit();
                                }

                                // This happens when the a frame takes too long to present
                                Err(wgpu::SurfaceError::Timeout) => {
                                    log::warn!("Surface timeout")
                                }
                            }
                        }
                        _ => {}
                    }
                }
            })
            .unwrap();
    });
    ext_objects
}
