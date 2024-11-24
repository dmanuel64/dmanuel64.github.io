use leptos::*;
use thaw::*;
use icondata_ai;

#[component]
pub fn NavItem(text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <Button icon=icondata_ai::AiHomeFilled variant=ButtonVariant::Link>
            <a href=href>{text}</a>
        </Button>
    }
}

#[component]
pub fn NavBar(children: Children) -> impl IntoView {
    view! {
        <Layout content_class="bg-gray-800 flex justify-end items-center h-16 px-4">
            {children()}
        </Layout>
    }
}


