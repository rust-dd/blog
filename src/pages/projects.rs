use dioxus::prelude::*;

use crate::{
    app::Route,
    components::{
        project_entry::ProjectEntry,
        shell::{SectionHeading, SideGroup, Sidebar, CONTENT_ID},
    },
    projects::{EXPERIMENTS, FEATURED, PAUSED, PRODUCTS},
    search::SearchQuery,
    seo,
};

#[component]
pub fn Component() -> Element {
    rsx! {
        seo::PageMeta {
            title: seo::page_title("Rust Projects, Apps and Developer Tools"),
            description: "Explore Rust-DD projects: stochastic-rs, Tako, embedded-dht-rs, rsql, candding, metal-oxide, React Native libraries, TurboEncode and the Amnézia game restoration.",
            path: "/projects",
        }
        Sidebar {
            SideGroup { title: "Sections",
                li { a { href: "#featured", class: "side-link", "Featured projects" } }
                li { a { href: "#products", class: "side-link", "Products & tools" } }
                li { a { href: "#experiments", class: "side-link", "Games & experiments" } }
                li { a { href: "#paused", class: "side-link", "On pause" } }
            }
            SideGroup { title: "Featured",
                for project in FEATURED {
                    li { a { href: "#{project.name}", class: "side-link side-code text-mod", "{project.name}" } }
                }
            }
            SideGroup { title: "Products & tools",
                for project in PRODUCTS {
                    li { a { href: "#{project.name}", class: "side-link side-code text-mod", "{project.name}" } }
                }
            }
            SideGroup { title: "Games & experiments",
                for project in EXPERIMENTS {
                    li { a { href: "#{project.name}", class: "side-link side-code text-mod", "{project.name}" } }
                }
            }
        }
        main { id: CONTENT_ID, tabindex: "-1", class: "shell-main",
            p { class: "doc-path",
                Link { to: Route::Home { query: SearchQuery::default() }, class: "text-mod", "rust_dd" }
                "::"
                span { class: "text-mod", "projects" }
            }
            div { class: "doc-title",
                h1 { class: "doc-h1",
                    "Module "
                    span { class: "text-mod", "projects" }
                }
                Link { to: Route::OpenSource {}, class: "doc-source", "Open-source index" }
            }
            p { class: "doc-lead",
                "Rust libraries, applications and experiments from Rust-DD."
            }

            section { aria_labelledby: "featured",
                SectionHeading { id: "featured", title: "Featured projects" }
                div { class: "project-list project-list-featured",
                    for project in FEATURED {
                        ProjectEntry { project }
                    }
                }
            }
            section { aria_labelledby: "products",
                SectionHeading { id: "products", title: "Products & tools" }
                div { class: "project-list",
                    for project in PRODUCTS {
                        ProjectEntry { project }
                    }
                }
            }
            section { aria_labelledby: "experiments",
                SectionHeading { id: "experiments", title: "Games & experiments" }
                div { class: "project-list",
                    for project in EXPERIMENTS {
                        ProjectEntry { project }
                    }
                }
            }
            section { aria_labelledby: "paused",
                SectionHeading { id: "paused", title: "On pause" }
                div { class: "project-list",
                    for project in PAUSED {
                        ProjectEntry { project, paused: true }
                    }
                }
            }
        }
    }
}
