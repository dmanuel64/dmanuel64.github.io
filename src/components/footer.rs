use leptos::*;
use thaw::*;

/// NavBar background container
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <Flex
            vertical=true
            class="absolute bottom-0 w-full bg-gray-800 text-white py-4 text-center"
        >
        ""
        </Flex>
        <div class="h-16"></div>
    }
}
