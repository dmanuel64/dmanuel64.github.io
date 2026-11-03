use leptos::prelude::*;
use thaw::*;

#[component]
pub fn Wedding() -> impl IntoView {
    let selected_value = RwSignal::new(String::new());
    view! {
        <Layout>
            <LayoutHeader attr:style="background-color: #ee4c49aa; padding: 20px;">
                <Flex vertical=true align=FlexAlign::Center>
                    <Text style="font-family: 'Signaline Script';" tag=TextTag::H1>
                        "Sarah & Dylan"
                    </Text>
                    <TabList selected_value>
                        <Tab value="our-story">"Our Story"</Tab>
                        <Tab value="save-the-date">"Save The Date"</Tab>
                        <Tab value="wedding-party">"Wedding Party"</Tab>
                        <Tab value="attire">"Attire"</Tab>
                        <Tab value="photos">"Photos"</Tab>
                        <Tab value="faqs">"FAQs"</Tab>
                        <Tab value="rsvp">"RSVP"</Tab>
                        <Tab value="registry">"Registry"</Tab>
                    </TabList>
                </Flex>
            </LayoutHeader>
            <Layout attr:style="background-color: #4ce05388; padding: 20px;">"Content"</Layout>
        </Layout>
    }
}
