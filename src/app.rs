use leptos::prelude::*;
use leptos_router::{
    components::*,
    hooks::{use_navigate, use_url},
    path,
};
use thaw::*;

use crate::pages::*;

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
            on:click=move |ev| {
                ev.prevent_default();
                if current_url.get().path() != navigate_url.as_str() {
                    loading_bar.start();
                    navigate(navigate_url.as_str(), Default::default());
                }
            }
        >
            <a
                class="text-inherit no-underline hover:text-inherit active:text-inherit visited:text-inherit"
                href=href
            >
                {value}
            </a>
        </Tab>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <Flex class="nav-bar" align=FlexAlign::Center justify=FlexJustify::SpaceBetween>
            <TabList>
                <Flex class="nav-items" justify=FlexJustify::SpaceAround>
                    <NavItem value="Home" href="/" />
                    <NavItem value="Blog" href="/blog" />
                    <NavItem value="Resume" href="/resume" />
                    <NavItem value="Contact" href="/contact" />
                </Flex>
            </TabList>
            <Flex class="nav-external-links" justify=FlexJustify::End>
                <Text>"External links"</Text>
            </Flex>
        </Flex>
    }
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
                        <footer></footer>
                    </LoadingBarProvider>
                </ConfigProvider>
            </Router>
        </div>
    }
}
