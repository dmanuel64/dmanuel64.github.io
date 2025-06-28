use leptos::prelude::*;
use leptos_router::{components::*, path};
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
    view! {
        <Tab class="nav-item" value=id>
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
    let selected_value = RwSignal::new(String::new());
    view! {
        <Flex class="nav-bar" align=FlexAlign::Center justify=FlexJustify::SpaceBetween>
            <TabList selected_value>
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
                    <nav>
                        <NavBar />
                    </nav>
                    <main>
                        <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                            <Route path=path!("/") view=Home />
                        // <Route path=path!("/blog") view=Users />
                        // <Route path=path!("/blog/:id") view=UserProfile />
                        </Routes>
                    </main>
                    <footer></footer>
                </ConfigProvider>
            </Router>
        </div>
    }
}
