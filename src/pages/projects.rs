use leptos::{logging, prelude::*};
use thaw::*;
use std::fs;

#[component]
fn ProjectPreview(
    #[prop(into)] name: String,
    #[prop(into)] description: String,
    #[prop(into)] href: String,
    #[prop(into, optional)] tags: Vec<String>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Body1>
                    <b>{name}</b>
                    {tags
                        .into_iter()
                        .map(|t| view! { <Tag>{t.to_lowercase()}</Tag> })
                        .collect::<Vec<_>>()}
                </Body1>
                <CardHeaderDescription slot>
                    <Caption1>{description}</Caption1>
                </CardHeaderDescription>
                <CardHeaderAction slot>
                    <Button
                        appearance=ButtonAppearance::Transparent
                        icon=icondata_ai::AiMoreOutlined
                    />
                </CardHeaderAction>
            </CardHeader>
            <CardPreview>
                <Image
                    src="https://github.com/dmanuel64/codablellm/blob/main/docs/images/quickstart_workflow.png?raw=true"
                    width="100%"
                />
            </CardPreview>
            <CardFooter>
                <Button>"View More"</Button>
            </CardFooter>
        </Card>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    let loading_bar = LoadingBarInjection::expect_context();
    loading_bar.finish();
    let tags = vec![
        "ai",
        "cybersecurity",
        "software engineering",
        "dataset curation",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
    use crate::pages::home::LONG_STR;
    view! {
        <Flex vertical=true align=FlexAlign::Center>
            <h1>"Projects"</h1>
            <Grid class="project-previews" cols=2>
                <ProjectPreview name="CodableLLM" description=LONG_STR href="" tags />
            </Grid>
        </Flex>
    }
}
