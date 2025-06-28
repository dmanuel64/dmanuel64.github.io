use leptos::prelude::*;
use leptos_router::{components::*, path};
use thaw::*;

use crate::pages::*;

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <Flex>
            <Text>"Hello"</Text>
        </Flex>
    }
}
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
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
            </Router>
        </div>
    }
}
