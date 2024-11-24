use leptos::*;
use thaw::*;

use crate::components::navbar::NavBar;

#[component]
fn NavCard(title: &'static str, href: &'static str, children: Children) -> impl IntoView {
    view! {
        <a href=href>
            <Layout
                content_class="rounded-3xl shadow-lg bg-white overflow-hidden text-center"
                class="border border-gray-200 rounded-3xl shadow-lg bg-white overflow-hidden transform transition-transform duration-200 hover:scale-105 animate-fadeInMoveUp"
            >
                <LayoutHeader class="relative -m-6 -mt-6">
                    <img
                        class="w-full h-auto"
                        src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg"
                        alt="Card image"
                    />
                </LayoutHeader>
                <Layout class="mt-4 p-6">
                    <LayoutHeader class="text-lg font-semibold text-gray-800">{title}</LayoutHeader>
                    <Text class="mt-2 text-gray-600 whitespace-normal break-words">
                        {children()}
                    </Text>
                </Layout>
            </Layout>
        </a>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <NavBar />
        <Flex align=FlexAlign::Center vertical=true>
            <Layout class="text-center animate-fadeIn">
                <LayoutHeader>"Welcome to my Website"</LayoutHeader>
                "Here is some text"
            </Layout>
            <Grid cols=3 x_gap=8 y_gap=8>
                <GridItem class="delay-1000">
                    <NavCard title="Home" href="/">"To Home page"</NavCard>
                </GridItem>
                <GridItem class="delay-1000">
                    <NavCard title="Resume" href="/resume">"To Resume page"</NavCard>
                </GridItem>
                <GridItem class="delay-1000">
                    <NavCard title="Contact" href="/contact">"To Contact page"</NavCard>
                </GridItem>
            </Grid>
        </Flex>
    }
}
