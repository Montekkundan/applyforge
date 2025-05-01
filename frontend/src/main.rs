mod app;
mod models;
mod login_register;

use app::AppRoot;

fn main() {
    yew::Renderer::<AppRoot>::new().render();
}