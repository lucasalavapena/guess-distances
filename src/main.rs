mod app;
mod components;
mod geo;
mod state;
mod traits;

use yew::prelude::*;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
