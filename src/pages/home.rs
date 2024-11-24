use leptos::*;
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <Layout class="my-0 mx-auto max-w-3xl text-center animate-fadeIn">
            <LayoutHeader class="p-6 text-4xl">"Welcome to my Website"</LayoutHeader>
                <Text class="block px-10 pb-10 text-center">"Hello, world"</Text>
                <Button
                    class="block mx-auto bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                    on:click=move |_| set_count.update(|count| *count += 1)
                >
                    {move || if count() == 0 {
                        "Click me!".to_string()
                    } else {
                        count().to_string()
                    }}
                </Button>
        </Layout>
    }
}
