mod app;
mod components;
mod geo;
mod state;
mod traits;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
