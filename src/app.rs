use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
                <nav></nav>
                <main>
                    // all our routes will appear inside <main>
                    <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                        <Route path=path!("/") view=Home />
                        // <Route path=path!("/blog") view=Users />
                        // <Route path=path!("/blog/:id") view=UserProfile />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}