use leptos::*;
use thaw::*;

use crate::components::navbar::{NavBar, NavItem};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <NavBar>
            <NavItem text="Home" href="/"/>
        </NavBar>
    }
}
