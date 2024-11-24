use leptos::*;
use thaw::*;

use crate::components::navbar::NavBar;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <NavBar/>
    }
}
