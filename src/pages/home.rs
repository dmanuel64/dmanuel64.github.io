use icondata_core::Icon;
use leptos::prelude::*;
use thaw::*;

const LONG_STR: &str = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed at sem sed libero tincidunt gravida. Morbi tempor posuere est, sed blandit sapien gravida a. Vivamus pretium libero at nisi dictum, sed bibendum justo congue. Phasellus tincidunt enim sit amet faucibus pretium. Fusce hendrerit, sapien vitae finibus gravida, lorem lacus ullamcorper libero, et tincidunt sapien nulla in enim. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Integer id semper justo. Nullam porttitor, metus ac euismod euismod, lectus erat pulvinar neque, nec efficitur ligula leo nec diam. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Integer sed erat nec mi feugiat malesuada. Suspendisse at efficitur arcu, nec pretium ex. Suspendisse ut nisl in libero blandit faucibus nec non nisl. Duis ut nunc eros. Curabitur hendrerit tellus nec ex iaculis, sed gravida mi tincidunt. Nunc nec viverra erat, at imperdiet purus. In id tellus vitae orci malesuada posuere. Etiam mattis quam id tortor varius, ut convallis felis dapibus. Etiam quis odio eget elit convallis posuere. Aliquam erat volutpat. Vestibulum congue gravida justo. Vivamus lacinia facilisis leo, sed facilisis metus vestibulum nec. Pellentesque et diam lacinia, dapibus erat sed, blandit nulla. Nam pulvinar nisl sit amet nisl pretium, a dictum sapien mattis. Integer feugiat laoreet lacus, sit amet feugiat turpis congue id. Sed mattis, ligula vel hendrerit bibendum, neque justo blandit risus, at gravida velit nulla eget neque. Morbi porttitor eu elit nec pharetra. Suspendisse potenti. Ut a dictum mi. Proin ut dignissim sem. Aenean vestibulum accumsan ex, et porttitor augue dignissim ut. Proin scelerisque tellus sit amet arcu porttitor, non posuere massa consequat. Vivamus tincidunt sem at facilisis laoreet. Nam tristique, arcu sed vehicula porta, nisl lorem dapibus justo, at pulvinar erat velit sit amet lorem.
";

#[component]
fn TechStackItem(#[prop(into)] name: String, #[prop(into)] icon: Icon) -> impl IntoView {
    view! {
        <GridItem class="tech-stack-item">
            <Flex vertical=true align=FlexAlign::Center justify=FlexJustify::Center>
                <Icon icon=icon width="100px" height="100px" />
                <Text style="font-size: 1.25em;">{name}</Text>
            </Flex>
        </GridItem>
    }
}

#[component]
fn TechStack() -> impl IntoView {
    view! {
        <Grid cols=4 x_gap=12 y_gap=12>
            // TODO: Rust
            <TechStackItem name="Python" icon=icondata_ai::AiPythonOutlined />
            <TechStackItem name="Docker" icon=icondata_ai::AiDockerOutlined />
            // TODO: CI/CD
            // TODO: Large Language Models
            <TechStackItem name="JavaScript" icon=icondata_ai::AiJavaScriptOutlined />
            // TODO: TypeScript
            // TODO: AWS
        </Grid>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let loading_bar = LoadingBarInjection::expect_context();
    loading_bar.finish();
    view! {
        <Flex
            class="home"
            style="min-height: inherit;"
            vertical=true
            align=FlexAlign::Center
            justify=FlexJustify::Center
        >
            <Flex class="home-introduction" align=FlexAlign::Center justify=FlexJustify::Center>
                <Image
                    src="/images/profile.png"
                    width="200px"
                    height="200px"
                    shape=ImageShape::Circular
                />
                <Flex class="home-about-me" vertical=true>
                    <Text style="font-size: 1.5em;">
                        <b>"Hi, my name is"</b>
                    </Text>
                    <Text style="font-size: 2em;">
                        <strong>
                            <b>"Dylan Manuel"</b>
                        </strong>
                    </Text>
                    <Text style="max-width: 200px;">
                        "
                        I'm a software engineer based in Houston, Texas. I specialize in building 
                        full-stack applications.
                        "
                    </Text>
                </Flex>
            </Flex>
            // TODO: create CSS for headers
            <h1 style="font-size: 1.75em; font-weight: 900; margin-top: 2%">"Tech Stack"</h1>
            <TechStack />
        </Flex>
    }
}
