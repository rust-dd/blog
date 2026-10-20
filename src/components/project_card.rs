use dioxus::prelude::*;

use crate::projects::Project;

#[component]
pub fn ProjectCard(
    project: &'static Project,
    #[props(default)] number: Option<usize>,
    #[props(default)] stars: Option<u32>,
    #[props(default)] paused: bool,
) -> Element {
    let class = if paused {
        "project-card project-card-paused"
    } else {
        "project-card"
    };

    rsx! {
        article {
            id: project.name,
            class,
            aria_labelledby: "{project.name}-title",
            div { class: "project-card-meta",
                p { class: "project-kind",
                    if let Some(number) = number {
                        span { class: "project-number", aria_hidden: "true", "{number:02} / " }
                    }
                    "{project.kind}"
                }
                if paused {
                    span { class: "project-status", "Temporarily paused" }
                } else if let Some(stars) = stars {
                    span { class: "project-stars", aria_label: "{stars} GitHub stars",
                        span { aria_hidden: "true", "☆ " }
                        "{stars}"
                    }
                }
            }
            h3 { id: "{project.name}-title", class: "project-name",
                if paused {
                    "{project.name}"
                } else {
                    a { href: project.url, rel: "noopener noreferrer", target: "_blank",
                        "{project.name}"
                    }
                }
            }
            p { class: "project-description", "{project.description}" }
            ul { class: "project-highlights", aria_label: "Highlights",
                for highlight in project.highlights {
                    li { "{highlight}" }
                }
            }
            if !paused {
                div { class: "project-links",
                    a {
                        href: project.url,
                        rel: "noopener noreferrer",
                        target: "_blank",
                        aria_label: "{project.action}: {project.name}",
                        "{project.action}"
                        span { aria_hidden: "true", " ↗" }
                    }
                    if let Some(repository) = project.repository {
                        a {
                            href: repository,
                            rel: "noopener noreferrer",
                            target: "_blank",
                            class: "project-source",
                            aria_label: "View source: {project.name}",
                            "Source"
                            span { aria_hidden: "true", " ↗" }
                        }
                    }
                }
            }
        }
    }
}
