mod app;
mod chip8;

use chip8::{HEIGHT, WIDTH};
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder};
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

    pixels.render().unwrap();

    let mut i = 0_u64;

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(winit::event::VirtualKeyCode::A) {
                println!("A");
            }
        }
        for pixel in pixels.frame_mut().chunks_exact_mut(4) {
            if let [r, g, b, a] = pixel {
                *r = i.try_into().unwrap();
                *g = i.try_into().unwrap();
                *b = i.try_into().unwrap();
                *a = 0xFF;
            }
        }
        i = (i + 1) % 255;

        pixels.render().unwrap();

        // *control_flow = ControlFlow::Exit;
    })

    // let bytes = include_bytes!("../data/2-ibm-logo.ch8");
    // println!("{:x?}", bytes);
}

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
