mod app;
mod models;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
