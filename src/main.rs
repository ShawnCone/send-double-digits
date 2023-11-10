mod components {
    pub mod app;
}

use leptos::*;
use send_double_digits::components::app::App;

fn main() {
    mount_to_body(App)
}
