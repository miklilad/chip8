use gloo::console::log;
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let canvas = use_node_ref();

    {
        let div_ref = canvas.clone();

        use_effect_with_deps(
            |div_ref| {
                let div = div_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("div_ref not attached to div element");

                let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(|_| {
                    web_sys::console::log_1(&"Clicked!".into());
                }));

                log!("Added event listener");
                div.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
                    .unwrap();

                move || {
                    log!("Cleaning up");
                    div.remove_event_listener_with_callback(
                        "click",
                        listener.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                }
            },
            div_ref,
        );
    }

    html! {
        <main>
            <canvas ref={canvas} id="canvas" width=200 height=200 style="border:1px solid #000000;"/>
        </main>
    }
}

fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .expect("Couldn't find canvas");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
