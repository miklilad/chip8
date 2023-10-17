mod app;
mod chip8;

use std::{sync::Arc, time::Instant};

use chip8::{HEIGHT, WIDTH};
use pixels::{Pixels, SurfaceTexture};
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

    let start_instant = Instant::now();
    // let game = (input, pixels, Instant::now());

    // game_loop::game_loop(
    //     event_loop,
    //     Arc::new(window),
    //     game,
    //     2,
    //     0.1,
    //     |gg| {
    //         let pixels = &mut gg.game.1;
    //         for pixel in pixels.frame_mut().chunks_exact_mut(4) {
    //             if let [r, g, b, a] = pixel {
    //                 *r = Instant::now()
    //                     .duration_since(gg.game.2)
    //                     .subsec_nanos()
    //                     .to_be_bytes()
    //                     .last()
    //                     .unwrap()
    //                     .to_owned();
    //                 *g = Instant::now()
    //                     .duration_since(gg.game.2)
    //                     .subsec_nanos()
    //                     .to_be_bytes()
    //                     .last()
    //                     .unwrap()
    //                     .to_owned();
    //                 *b = Instant::now()
    //                     .duration_since(gg.game.2)
    //                     .subsec_nanos()
    //                     .to_be_bytes()
    //                     .last()
    //                     .unwrap()
    //                     .to_owned();
    //                 *a = 0xFF;
    //             }
    //         }
    //     },
    //     |g| {
    //         g.game.1.render().unwrap();
    //     },
    //     |g, event| {
    //         let input = &mut g.game.0;
    //         if input.update(&event) {
    //             if input.close_requested() || input.key_pressed(VirtualKeyCode::Escape) {
    //                 g.exit();
    //             }
    //         }
    //     },
    // )

    let mut last_redraw_instant = Instant::now();

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
                for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                    if let [r, g, b, a] = pixel {
                        *r = Instant::now()
                            .duration_since(start_instant)
                            .subsec_nanos()
                            .to_be_bytes()
                            .last()
                            .unwrap()
                            .to_owned();
                        *g = Instant::now()
                            .duration_since(start_instant)
                            .subsec_nanos()
                            .to_be_bytes()
                            .last()
                            .unwrap()
                            .to_owned();
                        *b = Instant::now()
                            .duration_since(start_instant)
                            .subsec_nanos()
                            .to_be_bytes()
                            .last()
                            .unwrap()
                            .to_owned();
                        *a = 0xFF;
                    }
                }
                pixels.render().unwrap();
            }
            _ => {}
        }

        // println!("{:?}", event);
        if Instant::now()
            .duration_since(last_redraw_instant)
            .as_millis()
            > 1000
        {
            window.request_redraw();
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
