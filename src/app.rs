use leptos::*;
use leptos_router::*;

use crate::pages::{home::Home, not_found::NotFound, resume::Resume};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="/resume" view=Resume/>
                        <Route path="/*" view=NotFound/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
