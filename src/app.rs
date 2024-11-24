use leptos::*;
use leptos_router::*;

use crate::pages::{home::Home, not_found::NotFound};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="/*" view=NotFound/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
