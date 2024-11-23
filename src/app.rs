use leptos::*;
use leptos_router::*;

use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="/*" view=|| view! { <h1>"Not Found"</h1> }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
