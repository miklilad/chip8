mod chip8;

use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::chip8::{Chip8, HEIGHT, WIDTH};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WIDTH as u32 * 10, HEIGHT as u32 * 10))
        .with_title("Chip8")
        .build(&event_loop)
        .unwrap();

    let mut input = WinitInputHelper::new();

    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap();

    let mut last_redraw_instant = Instant::now();

    const UPS: u32 = 10;
    let time_step = 1.0 / (UPS as f64);

    let rom = include_bytes!("../data/IBM_Logo.ch8");

    let mut chip8 = Chip8::new(rom, chip8::Chip8Implementation::Modern);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                let display = chip8.display.concat();
                for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                    if let [r, g, b, a] = pixel {
                        *r = 0xFF * (display[i] & 1);
                        *g = 0xFF * (display[i] & 1);
                        *b = 0xFF * (display[i] & 1);
                        *a = 0xFF * (display[i] & 1);
                    }
                }
                pixels.render().unwrap();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput { input, .. } => {
                    for (i, scancode) in chip8.key_mapping.iter().enumerate() {
                        if input.scancode != *scancode {
                            continue;
                        }
                        match input.state {
                            ElementState::Pressed => {
                                chip8.keys_pressed[i] = true;
                            }
                            ElementState::Released => {
                                chip8.keys_pressed[i] = false;
                            }
                        }
                    }
                    if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                        control_flow.set_exit();
                    }
                    if input.virtual_keycode == Some(VirtualKeyCode::Space)
                        && input.state == ElementState::Pressed
                    {
                        window.request_redraw();
                    }
                }
                _ => {}
            },
            _ => {}
        }

        if Instant::now()
            .duration_since(last_redraw_instant)
            .as_secs_f64()
            > time_step
        {
            // window.request_redraw();
            if chip8.step() {
                window.request_redraw();
            }
            last_redraw_instant = Instant::now();
        }
    });
}
