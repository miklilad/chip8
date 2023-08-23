mod app;

use app::App;

fn main() {
    let bytes = include_bytes!("../data/2-ibm-logo.ch8");
    println!("{:x?}", bytes);
    // yew::Renderer::<App>::new().render();
}
