mod app;
mod chip8;

use std::{
    sync::Arc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use chip8::{HEIGHT, WIDTH};
use pixels::{wgpu::Instance, Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WIDTH * 10, HEIGHT * 10))
        .with_title("Chip8")
        .build(&event_loop)
        .unwrap();

    let mut input = WinitInputHelper::new();

    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    // pixels.clear_color(Color::RED);

    // for pixel in pixels.frame_mut() {
    //     *pixel = 0xFF;
    // }

    let game = (input, pixels, Instant::now());

    game_loop::game_loop(
        event_loop,
        Arc::new(window),
        game,
        60,
        0.1,
        |gg| {
            println!("Update called");
            let pixels = &mut gg.game.1;
            for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                if let [r, g, b, a] = pixel {
                    *r = Instant::now()
                        .duration_since(gg.game.2)
                        .subsec_nanos()
                        .to_be_bytes()
                        .last()
                        .unwrap()
                        .to_owned();
                    *g = Instant::now()
                        .duration_since(gg.game.2)
                        .subsec_nanos()
                        .to_be_bytes()
                        .last()
                        .unwrap()
                        .to_owned();
                    *b = Instant::now()
                        .duration_since(gg.game.2)
                        .subsec_nanos()
                        .to_be_bytes()
                        .last()
                        .unwrap()
                        .to_owned();
                    *a = 0xFF;
                }
            }
        },
        |g| {
            println!("Render called");
            g.game.1.render().unwrap();
        },
        |g, event| {},
    )

    // event_loop.run(move |event, _, control_flow| {
    //     println!("{i}");
    //     control_flow.set_wait();
    //     if input.update(&event) {
    //         if input.close_requested() || input.key_pressed(VirtualKeyCode::Escape) {
    //             *control_flow = ControlFlow::Exit;
    //         }
    //     }
    //     for pixel in pixels.frame_mut().chunks_exact_mut(4) {
    //         if let [r, g, b, a] = pixel {
    //             *r = i.try_into().unwrap();
    //             *g = i.try_into().unwrap();
    //             *b = i.try_into().unwrap();
    //             *a = 0xFF;
    //         }
    //     }
    //     i = (i + 1) % 255;

    //     if let Some(size) = input.window_resized() {
    //         pixels.resize_surface(size.width, size.height).unwrap();
    //     }

    //     pixels.render().unwrap();
    //     // pixels.render().unwrap();

    //     // *control_flow = ControlFlow::Exit;
    // })

    // let bytes = include_bytes!("../data/2-ibm-logo.ch8");
    // println!("{:x?}", bytes);
}

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
