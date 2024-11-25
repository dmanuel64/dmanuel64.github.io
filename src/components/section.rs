use leptos::*;
use thaw::*;

/// A button on the NavBar that navigates to a URL
#[component]
pub fn Section(
    /// The name of the section
    title: &'static str,
    /// Additional class variables to pass
    #[prop(default = "")]
    content_class: &'static str,
    /// Children components
    children: Children,
) -> impl IntoView {
    view! {
        <Layout content_class=content_class>
            <LayoutHeader class="text-lg font-semibold text-gray-800">
                <a href={format!("#{title}")}>
                    <Button
                        class=format!("text-black p-2 min-w-[40px]").as_str()
                        icon=icondata_ai::AiLinkOutlined
                        variant=ButtonVariant::Link
                    ></Button>
                </a>
                {title}
            </LayoutHeader>
            <div class="mt-2 text-gray-600 whitespace-normal break-words">{children()}</div>
        </Layout>
    }
}
