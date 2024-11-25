use leptos::*;
use thaw::*;

use crate::components::{navbar::NavBar, section::Section};

#[component]
pub fn Resume() -> impl IntoView {
    view! {
        <NavBar />
        <Flex vertical=true>
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
            <Divider />
            <Section title="Professional Experience">
                <Section title="Fullstack Software Engineer| BrainGu | 08/2022 - Present">
                    <ul></ul>
                </Section>
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
        </Flex>
    }
}
