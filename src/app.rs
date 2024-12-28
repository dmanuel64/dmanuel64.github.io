use leptos::*;
use leptos_router::*;

use crate::{
    components::{footer::Footer, navbar::NavBar},
    pages::{home::Home, not_found::NotFound, resume::Resume},
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="root" class="relative min-h-screen">
            <NavBar />
            <div class="pb-16">
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
            <Footer />
        </div>
    }
}
