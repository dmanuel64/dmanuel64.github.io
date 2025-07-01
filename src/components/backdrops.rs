use leptos::prelude::*;
use thaw::Theme;

#[component]
fn BackdropLight() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 1440 320"
            xmlns="http://www.w3.org/2000/svg"
            preserveAspectRatio="none"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;"
        >
            <defs>
                <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#f5f7fa" />
                    <stop offset="100%" stop-color="#c3cfe2" />
                </linearGradient>
            </defs>
            <rect width="100%" height="100%" fill="url(#grad)" />

            <path fill="#ffffff33">
                <animate
                    attributeName="d"
                    dur="10s"
                    repeatCount="indefinite"
                    values="
                    M0,160 C480,280 960,40 1440,160 L1440,320 L0,320 Z;
                    M0,120 C480,200 960,0 1440,120 L1440,320 L0,320 Z;
                    M0,160 C480,280 960,40 1440,160 L1440,320 L0,320 Z
                    "
                />
            </path>
        </svg>
    }
}

#[component]
fn BackdropDark() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 1440 320"
            xmlns="http://www.w3.org/2000/svg"
            preserveAspectRatio="none"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;"
        >
            <defs>
                <linearGradient id="darkGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#1e1e2f" />
                    <stop offset="100%" stop-color="#2c2c3a" />
                </linearGradient>
            </defs>
            <rect width="100%" height="100%" fill="url(#darkGrad)" />

            <path fill="#ffffff18">
                <animate
                    attributeName="d"
                    dur="12s"
                    repeatCount="indefinite"
                    values="
                    M0,160 C480,280 960,40 1440,160 L1440,320 L0,320 Z;
                    M0,120 C480,200 960,0 1440,120 L1440,320 L0,320 Z;
                    M0,160 C480,280 960,40 1440,160 L1440,320 L0,320 Z
                    "
                />
            </path>
        </svg>
    }
}

#[component]
pub fn Backdrop(theme: RwSignal<Theme>) -> impl IntoView {
    view! {
        <Show when=move || theme.get().name == "dark" fallback=BackdropLight>
            <BackdropDark />
        </Show>
    }
}
