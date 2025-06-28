use icondata_core::Icon;
use leptos::prelude::*;
use leptos_router::{
    components::*,
    hooks::{use_navigate, use_url},
    path,
};
use thaw::*;

use crate::pages::*;

const HIDDEN_LINK_TAILSCALE_CLASS: &str =
    "text-inherit no-underline hover:text-inherit active:text-inherit visited:text-inherit";
#[component]
fn NavItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] href: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    let id = if id == String::default() {
        value.to_lowercase().replace(" ", "-")
    } else {
        id
    };
    let navigate = use_navigate();
    let loading_bar = LoadingBarInjection::expect_context();
    let navigate_url = href.clone();
    let current_url = use_url();
    view! {
        <Tab
            class="nav-item"
            value=id
            on:click=move |_| {
                if current_url.get().path() != navigate_url.as_str() {
                    loading_bar.start();
                    navigate(navigate_url.as_str(), Default::default());
                }
            }
        >
            <a class=HIDDEN_LINK_TAILSCALE_CLASS href=href>
                {value}
            </a>
        </Tab>
    }
}

#[component]
fn ExternalLinkIcon(
    #[prop(into)] icon: Icon,
    #[prop(into, optional)] href: String,
    #[prop(into, default = String::from("2em"))] width: String,
    #[prop(into, default = String::from("2em"))] height: String,
) -> impl IntoView {
    view! {
        <a class="external-link-icon" href=href>
            <Icon class=HIDDEN_LINK_TAILSCALE_CLASS icon=icon width=width height=height />
        </a>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <Flex class="nav-bar" justify=FlexJustify::Center>
            <Flex
                style="
                width: 75%;
                border-radius: 8px;
                box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.2);
                "
                justify=FlexJustify::SpaceBetween
            >
                <TabList>
                    <Flex
                        class="nav-items"
                        align=FlexAlign::Center
                        justify=FlexJustify::SpaceAround
                    >
                        <NavItem value="Home" href="/" />
                        <NavItem value="Blog" href="/blog" />
                        <NavItem value="Resume" href="/resume" />
                        <NavItem value="Contact" href="/contact" />
                    </Flex>
                </TabList>
                <Flex
                    class="nav-external-links"
                    gap=FlexGap::Large
                    align=FlexAlign::Center
                    justify=FlexJustify::End
                >
                    <ExternalLinkIcon
                        icon=icondata_ai::AiGithubFilled
                        href="https://github.com/dmanuel64"
                    />
                    <ExternalLinkIcon
                        icon=icondata_ai::AiLinkedinFilled
                        href="https://www.linkedin.com/in/dylan-manuel-661642169"
                    />
                </Flex>
            </Flex>
        </Flex>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {}
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
                <ConfigProvider>
                    <LoadingBarProvider>
                        <nav>
                            <NavBar />
                        </nav>
                        <main>
                            <Routes fallback=|| NotFound>
                                <Route path=path!("/") view=Home />
                            // <Route path=path!("/blog") view=Users />
                            // <Route path=path!("/blog/:id") view=UserProfile />
                            </Routes>
                        </main>
                        <footer>
                            <Footer />
                        </footer>
                    </LoadingBarProvider>
                </ConfigProvider>
            </Router>
        </div>
    }
}
