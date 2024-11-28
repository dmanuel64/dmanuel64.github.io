use icondata_ai;
use icondata_core::IconData;
use leptos::*;
use thaw::*;

/// A button on the NavBar that navigates to a URL
#[component]
fn NavButton(
    /// The text to display
    text: &'static str,
    /// URL to navigate to
    href: &'static str,
    /// Icon to display next to the text
    #[prop(optional, into)]
    icon: Signal<Option<&'static IconData>>,
    /// Additional class variables to pass
    #[prop(default = "text-lg")]
    content_class: &'static str,
) -> impl IntoView {
    view! {
        <a href=href>
            <Button
                class=format!("text-white p-2 min-w-[40px] {content_class}").as_str()
                icon=icon
                variant=ButtonVariant::Link
            >
                {text}
            </Button>
        </a>
    }
}

/// A NavButton that is displayed by an icon instead of text
#[component]
fn NavIconButton(
    /// URL to navigate to
    href: &'static str,
    /// Icon to display
    #[prop(into)]
    icon: Signal<Option<&'static IconData>>,
) -> impl IntoView {
    view! { <NavButton text="" href=href icon=icon content_class="text-2xl" /> }
}

/// NavBar background container
#[component]
fn NavContainer(children: Children) -> impl IntoView {
    view! {
        <Flex class="bg-gray-800 fixed w-full items-center h-16 px-4 shadow-2xl z-50">
            {children()}
        </Flex>
        <div class="h-16"></div>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <NavContainer>
            <Flex class="space-x-4">
                <NavButton text="Home" href="/" icon=move || Some(icondata_ai::AiHomeFilled) />
                // <NavButton text="About" href="/about" />
                <NavButton text="Resume" href="/resume" />
                // <NavButton text="Projects" href="/projects" />
                // <NavButton text="Blog" href="/blog" />
                <NavButton text="Contact" href="/contact" />
            </Flex>
            <Flex class="ml-auto space-x-4">
                <NavIconButton
                    icon=move || Some(icondata_ai::AiGithubFilled)
                    href="https://github.com/dmanuel64"
                />
                <NavIconButton
                    icon=move || Some(icondata_ai::AiLinkedinFilled)
                    href="https://www.linkedin.com/in/dylan-manuel-661642169/"
                />
            </Flex>
        </NavContainer>
    }
}
