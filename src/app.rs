use crate::{components::Backdrop, pages::*};
use chrono::{Datelike, Utc};
use icondata_core::Icon;
use leptos::prelude::*;
use leptos_router::{
    components::*,
    hooks::{use_navigate, use_url},
    path,
};
use leptos_use::use_preferred_dark;
use thaw::*;

const HIDDEN_LINK_TAILSCALE_CLASS: &str =
    "text-inherit no-underline hover:text-inherit active:text-inherit visited:text-inherit";

fn get_nav_and_footer_background_color(theme: &Theme) -> String {
    theme.color.color_neutral_background_4.clone()
}

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
    let navigate = use_navigate();
    let loading_bar = LoadingBarInjection::expect_context();
    let navigate_url = href.clone();
    let current_url = use_url();
    view! {
        <Tab
            class="nav-item"
            value=id
            on:click=move |_| {
                if current_url.get().path() != navigate_url.as_str() {
                    loading_bar.start();
                    navigate(navigate_url.as_str(), Default::default());
                }
            }
        >
            <a class=HIDDEN_LINK_TAILSCALE_CLASS href=href>
                {value}
            </a>
        </Tab>
    }
}

#[component]
fn ExternalLinkIcon(
    #[prop(into)] icon: Icon,
    #[prop(into, optional)] href: String,
    #[prop(into, default = String::from("2em"))] width: String,
    #[prop(into, default = String::from("2em"))] height: String,
) -> impl IntoView {
    view! {
        <a class="external-link-icon" href=href>
            <Icon class=HIDDEN_LINK_TAILSCALE_CLASS icon width height />
        </a>
    }
}

#[component]
fn NavBar(theme: RwSignal<Theme>) -> impl IntoView {
    let checked = RwSignal::new(false);
    let navbar_style = Memo::new(move |_| {
        format!(
            "
                background-color: {};
                width: 97%;
                margin: 1%;
                padding: 0.5%;
                border-radius: 8px;
                box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.2);
                ",
            get_nav_and_footer_background_color(&theme.get())
        )
    });
    view! {
        <Flex class="nav-bar" justify=FlexJustify::Center>
            <Flex style=navbar_style justify=FlexJustify::SpaceBetween>
                <TabList>
                    <Flex
                        class="nav-items"
                        align=FlexAlign::Center
                        justify=FlexJustify::SpaceAround
                    >
                        <NavItem value="Home" href="/" />
                        <NavItem value="Blog" href="/blog" />
                        <NavItem value="Projects" href="/projects" />
                        <NavItem value="Contact" href="/contact" />
                    </Flex>
                </TabList>
                <Flex gap=FlexGap::Size(30)>
                    <Flex align=FlexAlign::Center>
                        <Show
                            when=move || checked.get()
                            fallback=|| {
                                view! {
                                    <Icon icon=icondata_ai::AiSunFilled width="2em" height="2em" />
                                }
                            }
                        >
                            <Icon icon=icondata_ai::AiMoonFilled width="2em" height="2em" />
                        </Show>
                        <Tooltip content="Toggle light/dark mode">
                            <Switch
                                checked
                                on:click=move |_| {
                                    theme
                                        .set(
                                            if !checked.get() { Theme::dark() } else { Theme::light() },
                                        );
                                }
                            />
                        </Tooltip>
                    </Flex>
                    <Flex
                        class="nav-external-links"
                        style="padding-right: 12px"
                        gap=FlexGap::Large
                        align=FlexAlign::Center
                    >
                        <ExternalLinkIcon
                            icon=icondata_ai::AiGithubFilled
                            href="https://github.com/dmanuel64"
                        />
                        <ExternalLinkIcon
                            icon=icondata_ai::AiLinkedinFilled
                            href="https://www.linkedin.com/in/dylan-manuel-661642169"
                        />
                    </Flex>
                </Flex>
            </Flex>
        </Flex>
    }
}

#[component]
fn Footer(theme: RwSignal<Theme>) -> impl IntoView {
    let footer_style = Memo::new(move |_| {
        format!(
            "
                background-color: {};
                height: inherit;
                ",
            get_nav_and_footer_background_color(&theme.get())
        )
    });
    view! {
        <Flex
            class="footer"
            style=footer_style
            vertical=true
            gap=FlexGap::Size(0)
            align=FlexAlign::Center
        >
            <Divider />
            <Flex
                vertical=true
                style="height: inherit;"
                align=FlexAlign::Center
                justify=FlexJustify::Center
            >
                <Text>
                    {format!("© {} Dylan Manuel. All rights reserved.", Utc::now().year())}
                </Text>
            </Flex>
        </Flex>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let is_dark_preferred = use_preferred_dark();
    let theme = RwSignal::new(if is_dark_preferred.get() {
        Theme::dark()
    } else {
        Theme::light()
    });
    view! {
        <Router>
            <ConfigProvider theme>
                <LoadingBarProvider>
                    <div style="position: relative; width: 100vw; height: 100vh;">
                        <Backdrop theme />
                        <Layout attr:id="root">
                            <LayoutHeader attr:style="z-index: 1000; position: fixed; top: 0; width: 100vw;">
                                <nav>
                                    <NavBar theme />
                                </nav>
                            </LayoutHeader>
                            <Layout attr:style="min-height: calc(100vh - 85px); margin-top: 85px; z-index: 0">
                                <main style="min-height: calc(100vh - 85px - 50px);">
                                    <Routes fallback=|| NotFound>
                                        <Route path=path!("/") view=Home />
                                    // <Route path=path!("/blog") view=Users />
                                    // <Route path=path!("/blog/:id") view=UserProfile />
                                    </Routes>
                                </main>
                                <footer style="height: 50px">
                                    <Footer theme />
                                </footer>
                            </Layout>
                        </Layout>
                    </div>
                </LoadingBarProvider>
            </ConfigProvider>
        </Router>
    }
}
