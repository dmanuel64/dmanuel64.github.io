use leptos::*;
use leptos_router::*;
use thaw::{use_loading_bar, LoadingBarProvider};

use crate::{components::navbar::NavBar, pages::{home::Home, not_found::NotFound, resume::Resume}};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="root">
            <NavBar />
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=Home />
                        <Route path="/resume" view=Resume />
                        <Route path="/*" view=NotFound />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
