use leptos::*;
use thaw::*;

use crate::components::navbar::NavBar;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <NavBar />
        <Text class="text-center align-middle justify-center text-5xl">"404 - Not Found"</Text>
    }
}
