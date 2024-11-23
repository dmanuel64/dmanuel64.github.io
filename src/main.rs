mod app;
mod components;
mod pages;

use app::App;
use leptos::*;

fn main() {
    #[cfg(feature = "dev")]
    {
        console_error_panic_hook::set_once();
    }
    leptos::mount_to_body(|| view! { <App /> })
}
