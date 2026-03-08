use dioxus::prelude::*;

use crate::{app::Route, seo};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Project {
    pub name: &'static str,
    pub kind: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub tags: &'static [&'static str],
}

pub const FEATURED_PROJECT: Project = Project {
    name: "rsql",
    kind: "database",
    description: "Fast PostgreSQL client built with Rust, Tauri, and React. Query data, inspect schema, run EXPLAIN, and stay responsive on large result sets.",
    url: "https://rsql.rust-dd.com/#demo",
    tags: &["postgresql", "rust", "tauri", "react"],
};

pub const PROJECTS: &[Project] = &[
    FEATURED_PROJECT,
    Project {
        name: "shrtn.ink",
        kind: "service",
        description: "Fast URL shortener for quick sharing with a minimal, no-noise interface.",
        url: "https://shrtn.ink/",
        tags: &["links", "web", "utility"],
    },
    Project {
        name: "stochasticlab",
        kind: "platform",
        description: "Cloud compute platform for simulation-heavy and quantitative workloads.",
        url: "https://stochasticlab.cloud/",
        tags: &["cloud", "compute", "quant"],
    },
    Project {
        name: "tryrust.org",
        kind: "education",
        description: "Interactive Rust tutorial and playground that runs directly in the browser.",
        url: "https://tryrust.org/",
        tags: &["rust", "learning", "browser"],
    },
    Project {
        name: "doom.rust-dd",
        kind: "experiment",
        description: "Playable browser experiment from Rust-DD built for fun and fast iteration.",
        url: "https://doom.rust-dd.com/",
        tags: &["browser", "game", "experiment"],
    },
];

#[component]
pub fn Component() -> Element {
    let title = "Projects | Rust-DD";
    let description = "Products, experiments, and live developer tools built by Rust-DD.";
    let canonical = seo::absolute_url("/projects");

    rsx! {
        document::Title { "{title}" }
        document::Meta { name: "description", content: "{description}" }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "googlebot", content: "index, follow" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "{title}" }
        document::Meta { property: "og:description", content: "{description}" }
        document::Meta { property: "og:url", content: "{canonical}" }
        document::Meta { property: "og:image", content: seo::DEFAULT_OG_IMAGE }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{title}" }
        document::Meta { name: "twitter:description", content: "{description}" }
        document::Meta { name: "twitter:url", content: "{canonical}" }
        document::Meta { name: "twitter:image", content: seo::DEFAULT_OG_IMAGE }
        document::Link { rel: "canonical", href: "{canonical}" }

        div { class: "w-full font-mono",
            section { class: "py-4",
                p { class: "text-xs text-slate-400", "// projects" }
                h1 { class: "mt-2 text-3xl font-semibold leading-tight text-slate-900 sm:text-4xl md:text-5xl",
                    "Projects"
                }
                p { class: "mt-3 max-w-2xl text-sm leading-relaxed text-slate-600",
                    "Live tools, experiments, and product work from Rust-DD."
                }
            }

            div { class: "mt-4 border-y border-dashed border-slate-300 py-3 text-xs text-slate-500",
                div { class: "flex flex-wrap gap-x-4 gap-y-1",
                    span {
                        "live: "
                        span { class: "text-slate-700", "{PROJECTS.len()}" }
                    }
                    span { class: "hidden sm:inline", "|" }
                    span {
                        "featured: "
                        span { class: "text-slate-700", "{FEATURED_PROJECT.name}" }
                    }
                    span { class: "hidden sm:inline", "|" }
                    span {
                        "more: "
                        Link {
                            to: Route::OpenSource {},
                            class: "text-slate-700 hover:text-slate-900",
                            "open source"
                        }
                    }
                }
            }

            div { class: "mt-6 rounded-xl border border-slate-900 bg-slate-950 p-5 text-slate-50 sm:p-6",
                div { class: "flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between",
                    div { class: "max-w-2xl",
                        p { class: "text-[11px] uppercase tracking-[0.24em] text-emerald-300", "featured project" }
                        h2 { class: "mt-3 text-2xl font-semibold tracking-tight sm:text-3xl",
                            "{FEATURED_PROJECT.name}"
                        }
                        p { class: "mt-3 text-sm leading-relaxed text-slate-300",
                            "{FEATURED_PROJECT.description}"
                        }
                        div { class: "mt-4 flex flex-wrap gap-2 text-[11px] text-slate-300",
                            for tag in FEATURED_PROJECT.tags.iter() {
                                span { class: "rounded-full border border-white/15 px-2 py-1", "{tag}" }
                            }
                        }
                    }
                    div { class: "flex shrink-0 flex-col gap-3 sm:items-end",
                        a {
                            href: "{FEATURED_PROJECT.url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center justify-center rounded-md bg-emerald-300 px-4 py-2 text-sm font-medium text-slate-950 transition-colors duration-200 hover:bg-emerald-200",
                            "Open demo"
                        }
                        Link {
                            to: Route::OpenSource {},
                            class: "inline-flex items-center justify-center rounded-md border border-white/15 px-4 py-2 text-sm text-slate-200 transition-colors duration-200 hover:border-white/30 hover:text-white",
                            "See open source repos"
                        }
                    }
                }
            }

            section { class: "mt-6 grid gap-4 md:grid-cols-2 xl:grid-cols-3",
                for project in PROJECTS.iter().filter(|project| project.name != FEATURED_PROJECT.name) {
                    ProjectCard { project: *project }
                }
                Link {
                    to: Route::OpenSource {},
                    class: "group flex min-h-[220px] flex-col justify-between rounded-xl border border-dashed border-slate-300 bg-slate-50 p-5 no-underline transition-colors duration-200 hover:border-slate-500 hover:bg-white",
                    div {
                        p { class: "text-[11px] uppercase tracking-[0.2em] text-slate-400", "index" }
                        h3 { class: "mt-3 text-xl font-semibold text-slate-900", "Open source repos" }
                        p { class: "mt-2 text-sm leading-relaxed text-slate-600",
                            "Browse libraries, frameworks, CLI tools, and public repos from the same workspace."
                        }
                    }
                    span { class: "text-sm text-slate-400 transition-colors duration-200 group-hover:text-slate-700", "open /opensource ->" }
                }
            }
        }
    }
}

#[component]
fn ProjectCard(project: Project) -> Element {
    rsx! {
        a {
            href: "{project.url}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "group flex min-h-[220px] flex-col justify-between rounded-xl border border-slate-200 bg-white p-5 transition-colors duration-200 hover:border-slate-400 hover:bg-slate-50",
            div {
                p { class: "text-[11px] uppercase tracking-[0.2em] text-slate-400", "{project.kind}" }
                h3 { class: "mt-3 text-xl font-semibold text-slate-900", "{project.name}" }
                p { class: "mt-2 text-sm leading-relaxed text-slate-600", "{project.description}" }
                div { class: "mt-4 flex flex-wrap gap-2 text-[11px] text-slate-500",
                    for tag in project.tags.iter() {
                        span { class: "rounded-full border border-slate-200 px-2 py-1", "{tag}" }
                    }
                }
            }
            span { class: "text-sm text-slate-400 transition-colors duration-200 group-hover:text-slate-700", "visit ->" }
        }
    }
}
