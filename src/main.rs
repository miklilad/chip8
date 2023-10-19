mod app;
mod chip8;

use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize, event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder,
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

    const UPS: u32 = 7;
    let time_step = 1.0 / (UPS as f64);

    let rom = include_bytes!("../data/2-ibm-logo.ch8");
    // let rom = include_bytes!("../data/IBM Logo.ch8");

    let mut chip8 = Chip8::new(rom);

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.close_requested()
                || input.key_pressed(VirtualKeyCode::Escape)
                || input.key_pressed(VirtualKeyCode::Q)
            {
                control_flow.set_exit();
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                window.request_redraw();
            }
        }

        match event {
            winit::event::Event::RedrawRequested(_) => {
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
            _ => {}
        }

        // println!("{:?}", event);
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

        // if let Some(size) = input.window_resized() {
        //     pixels.resize_surface(size.width, size.height).unwrap();
        // }

        // pixels.render().unwrap();
        // pixels.render().unwrap();

        // *control_flow = ControlFlow::Exit;
    });

    // let bytes = include_bytes!("../data/2-ibm-logo.ch8");
    // println!("{:x?}", bytes);
}

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
