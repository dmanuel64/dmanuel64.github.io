use leptos::prelude::*;
use leptos_router::{components::*, path};
use thaw::*;

use crate::pages::*;

#[component]
fn NavBar() -> impl IntoView {
    let selected_value = RwSignal::new(String::new());
    view! {
        <Flex align=FlexAlign::Center justify=FlexJustify::SpaceBetween>
            <TabList selected_value>
                <Flex justify=FlexJustify::SpaceAround>
                    <Tab value="home"><a href="/">"Home"</a></Tab>
                    <Tab value="resume">"Resume"</Tab>
                    <Tab value="contact">"Contact"</Tab>
                    <Tab value="blog">"Blog"</Tab>
                </Flex>
            </TabList>
            <Flex justify=FlexJustify::End>
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
