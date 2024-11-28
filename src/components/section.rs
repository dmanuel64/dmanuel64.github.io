use leptos::*;
use leptos_use::{use_clipboard, use_window, UseClipboardReturn};
use thaw::*;

/// A button on the NavBar that navigates to a URL
#[component]
pub fn Section(
    /// The name of the section
    title: &'static str,
    #[prop(default = true)] allow_copy: bool,
    /// Additional class variables to pass
    #[prop(default = "")]
    content_class: &'static str,
    /// Children components
    children: Children,
) -> impl IntoView {
    let message = use_message();
    let copy_link = move |_| {
        let UseClipboardReturn {
            is_supported,
            text: _,
            copied: _,
            copy,
        } = use_clipboard();
        if is_supported() {
            if let Some(url) = use_window()
                .as_ref()
                .and_then(|w| w.location().to_string().as_string())
            {
                copy(url.as_str());
                message.create(
                    "Link copied".into(),
                    MessageVariant::Success,
                    Default::default(),
                );
            } else {
                message.create(
                    "Could not copy link".into(),
                    MessageVariant::Error,
                    Default::default(),
                );
            }
        } else {
            message.create(
                "Your browser does not support clipboard copying".into(),
                MessageVariant::Error,
                Default::default(),
            );
        }
    };
    view! {
        <Layout content_class=content_class>
            <LayoutHeader class="text-lg font-semibold text-gray-800 ml-8 mt-4">
                <Show when=move || allow_copy>
                    <Popover>
                        <PopoverTrigger slot>
                            <a href=format!("#{title}")>
                                <Button
                                    class=format!("text-black min-w-[40px]").as_str()
                                    icon=icondata_ai::AiLinkOutlined
                                    variant=ButtonVariant::Link
                                    on_click=copy_link
                                ></Button>
                            </a>
                        </PopoverTrigger>
                        "Copy link"
                    </Popover>
                </Show>
                {title}
            </LayoutHeader>
            <div class="ml-20 text-gray-600 whitespace-normal break-words">{children()}</div>
        </Layout>
    }
}
