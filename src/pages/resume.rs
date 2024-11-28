use leptos::*;
use thaw::*;

use crate::components::{navbar::NavBar, section::Section};

const RUST_LOGO_URL: &str = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
const PYTHON_LOGO_URL: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/c/c3/Python-logo-notext.svg/800px-Python-logo-notext.svg.png";
const JAVASCRIPT_LOGO_URL: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/2048px-Unofficial_JavaScript_logo_2.svg.png";

#[component]
fn BadgeText(
    badge: &'static str,
    #[prop(default = "75px")] width: &'static str,
    #[prop(default = "75px")] height: &'static str,
    #[prop(optional)] before: bool,
    #[prop(optional)] popover: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let image_fragment = popover.map_or(
        view! { <Image src=badge width=width height=height /> },
        |p| {
            view! {
                <Popover>
                    <PopoverTrigger slot>
                        <Image src=badge width=width height=height />
                    </PopoverTrigger>
                    {p}
                </Popover>
            }
        },
    );
    let text_fragment = if before {
        view! {
            {image_fragment}
            {children()}
        }
    } else {
        view! {
            {children()}
            {image_fragment}
        }
    };
    view! { <Flex align=FlexAlign::Center>{text_fragment}</Flex> }
}

#[component]
fn TechStack() -> impl IntoView {
    view! {
        <Grid cols=4 x_gap=8 class="p-8">
            <GridItem>
                <Section title="Frontend">
                    <ul>
                        <li>"• Node.js"</li>
                        <li>"• Typescript"</li>
                        <li>
                            <BadgeText
                                badge=JAVASCRIPT_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="JavaScript frontend library"
                            >
                                "• React"
                            </BadgeText>
                        </li>
                        <li>
                            <BadgeText
                                badge=RUST_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Rust frontend library"
                            >
                                "• Leptos"
                            </BadgeText>
                        </li>
                        <li>
                            <BadgeText
                                badge=JAVASCRIPT_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="JavaScript component library"
                            >
                                "• Material UI"
                            </BadgeText>
                        </li>
                        <li>
                            <BadgeText
                                badge=RUST_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Rust component library"
                            >
                                "• Thaw UI"
                            </BadgeText>
                        </li>
                    </ul>
                </Section>
            </GridItem>
            <GridItem>
                <Section title="Backend">
                    <ul>
                        <li>
                            <BadgeText
                                badge=PYTHON_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Python web framework"
                            >
                                "• Django"
                            </BadgeText>
                        </li>
                        <li>
                            <BadgeText
                                badge=PYTHON_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Python Object Relational Mapper (ORM)"
                            >
                                "• SQLAlchemy"
                            </BadgeText>
                        </li>
                        <li>"• Apache Kafka"</li>
                        <li>"• Docker"</li>
                        <li>
                            <BadgeText
                                badge=PYTHON_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Python web framework"
                            >
                                "• Flask"
                            </BadgeText>
                        </li>
                        <li>
                            <BadgeText
                                badge=RUST_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Rust web framework"
                            >
                                "• Rocket"
                            </BadgeText>
                        </li>
                    </ul>
                </Section>
            </GridItem>
            <GridItem>
                <Section title="Machine Learning">
                    <ul>
                        <li>
                            <BadgeText
                                badge=PYTHON_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Python machine learning library"
                            >
                                "• PyTorch"
                            </BadgeText>
                        </li>
                        <li>"• Hugging Face"</li>
                        <li>"• Large Language Models"</li>
                        <li>"• Retrieval-Augmented Generation"</li>
                        <li>
                            <BadgeText
                                badge=PYTHON_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Python (also available in other languages) data plotting library"
                            >
                                "• Plotly"
                            </BadgeText>
                        </li>
                        <li>
                            <BadgeText
                                badge=PYTHON_LOGO_URL
                                width="25px"
                                height="25px"
                                popover="Python tabular data manipulation & analysis library"
                            >
                                "• Pandas"
                            </BadgeText>
                        </li>
                    </ul>
                </Section>
            </GridItem>
            <GridItem>
                <Section title="Tools & Platforms">
                    <ul>
                        <li>"• AWS"</li>
                        <li>"• Linux"</li>
                        <li>"• GitHub"</li>
                        <li>"• GitLab"</li>
                        <li>"• CI/CD"</li>
                    </ul>
                </Section>
            </GridItem>
        </Grid>
    }
}

#[component]
pub fn Resume() -> impl IntoView {
    view! {
        <MessageProvider>
            <Flex gap=FlexGap::Small vertical=true>
                <Section title="Summary">
                    "
                    Software engineer specializing in applied AI for cybersecurity, with \
                    extensive hands-on experience in developing and deploying machine learning \
                    models and scalable software solutions. Demonstrated success in delivering \
                    AI-driven systems that enhance efficiency and strengthen security. Eager to \
                    leverage advanced research and development expertise in a dynamic AI engineering role.
                    "
                </Section>
                <Divider />
                <Section title="Tech Stack">
                    <TechStack />
                </Section>
                <Divider />
                <Section title="Professional Experience">
                    <Section title="Fullstack Software Engineer| BrainGu | 08/2022 - Present">
                        <ul></ul>
                    </Section>
                    <Section title="Associate Software Engineer | Northrop Grumman Corporation | 05/2021 - 08/2022">
                        <ul>
                            <li>
                                "• Developed a MLOps tool for automating AI testing, leveraging Pandas, CI/CD, Python, and AWS"
                            </li>
                            <li>
                                "• Implemented infrastructure for deploying and monitoring AI models, \
                                ensuring compliance with ethical guidelines and industry standards, \
                                and utilizing CredoAI for responsible AI governance"
                            </li>
                            <li>
                                "• Used Jira and GitLab for project management, issue tracking, and version control"
                            </li>
                        </ul>
                    </Section>
                    <Section title="Cyber Technical Intern | Northrop Grumman Corporation | 06/2020 - 07/2020">
                        <ul>
                            <li>
                                "• Used Python, Pandas, and Matplotlib to automate log data processing and \
                                generate visualizations"
                            </li>
                            <li>
                                "• Streamlined cybersecurity metric analysis, enabling the team to focus on \
                                high-level analysis and decision-making"
                            </li>
                            <li>"• Reduced manual data processing and visualization tasks"</li>
                        </ul>

                    </Section>
                </Section>
                <Divider />
                <Section title="Certifications">
                    <ul>
                        <li>
                            <BadgeText
                                badge="https://images.credly.com/size/340x340/images/0e284c3f-5164-4b21-8660-0d84737941bc/image.png"
                                before=true
                            >
                                <a href="https://www.credly.com/badges/1a2c8da1-6ad9-4706-8e95-61d290a30bb2/linked_in_profile">
                                    <u>"AWS Certified Solutions Architect - Associate"</u>
                                </a>
                                " (ID: 5e3cbe6ff6714d7f85d60020bbb5dfd0, Expires: 02/02/2027)"
                            </BadgeText>
                        </li>
                    </ul>
                </Section>
                <Divider />
                <Section title="Education">
                    <Section title="The University of Texas at San Antonio | Master of Science | 01/2022 - 05/2025 (Expected)">
                        <ul>
                            <li>
                                "• Major: Computer Science, GPA: 3.94; Research area in applied AI for cyber applications"
                            </li>
                            <li>
                                "• Collaborated with Sandia National Labs and implemented an AI-based \
                                recommendation system using Doc2Vec and other data-driven algorithms to \
                                identify cybersecurity improvements for systems"
                            </li>
                            <li>
                                "• Developed a transformer-based model to detect source code \
                                vulnerabilities. Utilized Weights & Biases to manage datasets and \
                                models, and to monitor training/evaluation metrics"
                            </li>
                        </ul>
                    </Section>
                    <Section title="The University of Texas at San Antonio | Bachelor of Science | 08/2018 - 12/2021">
                        <ul>
                            <li>
                                "• Major: Computer Science, GPA: 3.89; Dual Concentration in Software Engineering and Data Science"
                            </li>
                        </ul>
                    </Section>
                </Section>
                <div class="p-2"/>
            </Flex>
        </MessageProvider>
    }
}
