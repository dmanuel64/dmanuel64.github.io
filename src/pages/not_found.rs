use leptos::prelude::*;
use thaw::*;

#[component]
pub fn NotFound() -> impl IntoView {
    let loading_bar = LoadingBarInjection::expect_context();
    loading_bar.error();
    view! { <h1>"Not Found"</h1> }
}
