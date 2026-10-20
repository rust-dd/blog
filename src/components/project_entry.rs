use dioxus::prelude::*;

use crate::projects::Project;

#[component]
pub fn ProjectEntry(
    project: &'static Project,
    #[props(default)] stars: Option<u32>,
    #[props(default)] paused: bool,
) -> Element {
    rsx! {
        article {
            id: project.name,
            class: "project-entry",
            aria_labelledby: "{project.name}-title",
            header { class: "project-heading",
                h3 { id: "{project.name}-title", class: "project-name",
                    a {
                        class: "doc-anchor",
                        href: "#{project.name}",
                        aria_label: "Link to {project.name}",
                        "§"
                    }
                    if paused {
                        "{project.name}"
                    } else {
                        a { href: project.url, rel: "noopener noreferrer", target: "_blank",
                            "{project.name}"
                        }
                    }
                }
                p { class: "project-meta",
                    span { "{project.kind}" }
                    if let Some(stars) = stars {
                        span { class: "project-stars", "{stars} stars" }
                    }
                }
            }
            div { class: "project-body",
                if paused {
                    p { class: "project-status", "Temporarily paused" }
                }
                p { class: "project-description", "{project.description}" }
                if !paused {
                    p { class: "project-links",
                        a {
                            href: project.url,
                            rel: "noopener noreferrer",
                            target: "_blank",
                            aria_label: "{project.action}: {project.name}",
                            "{project.action}"
                        }
                        if let Some(repository) = project.repository {
                            a {
                                href: repository,
                                rel: "noopener noreferrer",
                                target: "_blank",
                                aria_label: "View source: {project.name}",
                                "Source"
                            }
                        }
                    }
                }
            }
        }
    }
}
