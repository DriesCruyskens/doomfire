//! An implementation of doomfire using `pixels`.
//!
//! Press `Space` to extinguish/ignite the fire (Extinguishing is not as immediate as igniting) .
use doomfire::Doomfire;
use pixels::{wgpu::Surface, Error, PixelsBuilder, SurfaceTexture};
use std::thread;
use std::time::{Duration, Instant};
use winit::{
    dpi::{PhysicalSize, Size},
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const HEIGHT: usize = 200;
const WIDTH: usize = 600;
const TITLE: &str = "Doomfire";
const FPS: u64 = 60;

fn main() -> Result<(), Error> {
    // init window
    let event_loop = EventLoop::new();
    /*  For some reason using a logical size throws
        thread 'main' panicked at 'index out of bounds: the len is 2400 but the index is 2400'
        later in doomfire's draw() funtion.
    */
    //let size: Size = Size::Logical(LogicalSize::new(WIDTH as f64, HEIGHT as f64));
    let size: Size = Size::Physical(PhysicalSize::new(WIDTH as u32, HEIGHT as u32));
    let window = WindowBuilder::new()
        .with_inner_size(size)
        .with_title(TITLE)
        .build(&event_loop)
        .unwrap();

    // init pixels
    let surface = Surface::create(&window);
    // surface_texture expects physical size, luckily inner_size returns physicial size.
    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, surface);
    // request_adapter_options with LowPower (default I guess) throws exit code: 0xc0000005, STATUS_ACCESS_VIOLATION
    // fixes a driver issue https://github.com/parasyte/pixels/issues/49
    let mut pixels = PixelsBuilder::new(size.width, size.height, surface_texture)
        .request_adapter_options(wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
        })
        .build()?;

    let mut doomfire = Doomfire::new(WIDTH, HEIGHT);
    doomfire.ignite();

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                doomfire.draw(pixels.get_frame());
                if pixels.render().map_err(|e| Err::<(), Error>(e)).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                doomfire.update();

                // Max the redraw to 60 fps
                let end_time = Instant::now();
                let render_time = end_time - start_time;
                if render_time < Duration::from_millis(1000 / FPS) {
                    let waste_time = Duration::from_millis(1000 / FPS) - render_time;
                    thread::sleep(waste_time);
                }
            }
            Event::DeviceEvent {
                // Using pattern matching and destructuring to extract KeyboardInput.
                event: DeviceEvent::Key(keyboard_input),
                device_id: _,
            } => {
                // Using if let instead of match since we only have to handle Some.
                if let Some(key_code) = keyboard_input.virtual_keycode {
                    match key_code {
                        // Using a match guard to make sure we run code on pressed and not released.
                        VirtualKeyCode::Space if keyboard_input.state == ElementState::Pressed => {
                            if doomfire.is_lit {
                                doomfire.extinguish();
                            } else if !doomfire.is_lit {
                                doomfire.ignite();
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    });
}
