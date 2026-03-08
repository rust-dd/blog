use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::{components::loader, seo, ssr::api::select_repo_stars};

pub(crate) struct OssProject {
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) url: &'static str,
    pub(crate) github_repo: &'static str,
    pub(crate) stars: u32,
    pub(crate) language: &'static str,
    pub(crate) topics: &'static [&'static str],
}

pub(crate) const PROJECTS: &[OssProject] = &[
    OssProject {
        name: "tako",
        description: "Tako is a lightweight and minimalistic web framework built on Tokio and Hyper written in Rust.",
        url: "https://github.com/rust-dd/tako",
        github_repo: "rust-dd/tako",
        stars: 146,
        language: "Rust",
        topics: &["async", "hyper", "rust", "tokio", "webframework"],
    },
    OssProject {
        name: "stochastic-rs",
        description: "A Rust library designed for high-performance simulation and analysis of stochastic processes and models in quantitative finance.",
        url: "https://github.com/rust-dd/stochastic-rs",
        github_repo: "rust-dd/stochastic-rs",
        stars: 141,
        language: "Rust",
        topics: &["quant", "finance", "stochastic", "simulation", "statistics"],
    },
    OssProject {
        name: "rust-axum-async-graphql-postgres-redis-starter",
        description: "Starter template using Rust with Axum, Async-GraphQL, PostgreSQL, and Redis for building high-performance web APIs.",
        url: "https://github.com/rust-dd/rust-axum-async-graphql-postgres-redis-starter",
        github_repo: "rust-dd/rust-axum-async-graphql-postgres-redis-starter",
        stars: 42,
        language: "Rust",
        topics: &["axum", "graphql", "postgres", "redis"],
    },
    OssProject {
        name: "rsql",
        description: "Fast PostgreSQL client built with Rust, Tauri, and React for querying data, running EXPLAIN, and exploring large result sets.",
        url: "https://rsql.rust-dd.com/#demo",
        github_repo: "rust-dd/rust-sql",
        stars: 37,
        language: "TypeScript",
        topics: &["postgresql", "rust", "tauri", "react"],
    },
    OssProject {
        name: "embedded-dht-rs",
        description: "A Rust library that provides full support for DHT11, DHT22, and DHT20 (AHT20) temperature and humidity sensors.",
        url: "https://github.com/rust-dd/embedded-dht-rs",
        github_repo: "rust-dd/embedded-dht-rs",
        stars: 34,
        language: "Rust",
        topics: &["dht11", "dht22", "esp32", "embedded"],
    },
    OssProject {
        name: "aoc-2024",
        description: "Solving the Advent of Code 2024 puzzles using the Rust programming language.",
        url: "https://github.com/rust-dd/aoc-2024",
        github_repo: "rust-dd/aoc-2024",
        stars: 24,
        language: "Rust",
        topics: &["advent-of-code", "rust"],
    },
    OssProject {
        name: "iTransformer",
        description: "An iTransformer implementation in Rust for time-series forecasting.",
        url: "https://github.com/rust-dd/iTransformer",
        github_repo: "rust-dd/iTransformer",
        stars: 18,
        language: "Rust",
        topics: &["ai", "transformers", "mathematics"],
    },
    OssProject {
        name: "blog",
        description: "Blog engine written in Rust, powered by Dioxus and SurrealDB.",
        url: "https://github.com/rust-dd/blog",
        github_repo: "rust-dd/blog",
        stars: 16,
        language: "Rust",
        topics: &["blog", "dioxus", "surrealdb"],
    },
    OssProject {
        name: "google-calendar-cli",
        description: "Google Calendar CLI written in Rust.",
        url: "https://github.com/rust-dd/google-calendar-cli",
        github_repo: "rust-dd/google-calendar-cli",
        stars: 13,
        language: "Rust",
        topics: &["cli", "google-calendar", "rust"],
    },
    OssProject {
        name: "probability-benchmark",
        description: "Scientific computing benchmark: Rust vs Zig vs C.",
        url: "https://github.com/rust-dd/probability-benchmark",
        github_repo: "rust-dd/probability-benchmark",
        stars: 11,
        language: "Zig",
        topics: &["rust", "zig", "c", "stochastic-processes"],
    },
    OssProject {
        name: "tryrust.org",
        description: "An interactive Rust tutorial in the browser.",
        url: "https://github.com/rust-dd/tryrust.org",
        github_repo: "rust-dd/tryrust.org",
        stars: 8,
        language: "Rust",
        topics: &["axum", "leptos", "tutorial"],
    },
    OssProject {
        name: "async-safe-defer",
        description: "Minimal async- and sync-capable defer crate for Rust.",
        url: "https://github.com/rust-dd/async-safe-defer",
        github_repo: "rust-dd/async-safe-defer",
        stars: 7,
        language: "Rust",
        topics: &["async", "defer", "embedded"],
    },
];

#[component]
pub fn Component() -> Element {
    let stars = use_server_future(select_repo_stars)?;
    let title = "Open Source | Rust-DD";
    let description =
        "Open source projects by Rust-DD — web frameworks, quant finance, embedded systems, and developer tools.";
    let canonical = seo::absolute_url("/opensource");

    let mut languages: Vec<&str> = PROJECTS.iter().map(|p| p.language).collect();
    languages.sort();
    languages.dedup();

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

        SuspenseBoundary {
            fallback: |_| rsx! { loader::Inline { message: "Loading projects...".to_string() } },
            match stars.read().as_ref() {
                Some(result) => {
                    let repo_stars = result.as_ref().ok();
                    let total_stars: u32 = PROJECTS
                        .iter()
                        .map(|project| project_stars(project, repo_stars))
                        .sum();

                    rsx! {
                        div { class: "w-full font-mono",
                            section { class: "py-4",
                                p { class: "text-xs text-slate-400", "// open source" }
                                h1 { class: "mt-2 text-3xl font-semibold leading-tight text-slate-900 sm:text-4xl md:text-5xl",
                                    "Open Source"
                                }
                                p { class: "mt-3 max-w-2xl text-sm leading-relaxed text-slate-600",
                                    "Libraries, frameworks, and tools we build and maintain in the open."
                                }
                            }

                            div { class: "mt-4 border-y border-dashed border-slate-300 py-3 text-xs text-slate-500",
                                div { class: "flex flex-wrap gap-x-4 gap-y-1",
                                    span {
                                        "repos: "
                                        span { class: "text-slate-700", "{PROJECTS.len()}" }
                                    }
                                    span { class: "hidden sm:inline", "|" }
                                    span {
                                        "stars: "
                                        span { class: "text-slate-700", "{total_stars}" }
                                    }
                                    span { class: "hidden sm:inline", "|" }
                                    span {
                                        "org: "
                                        a {
                                            href: "https://github.com/rust-dd",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            class: "text-slate-700 hover:text-slate-900",
                                            "rust-dd"
                                        }
                                    }
                                }
                            }

                            div { class: "mt-4 text-xs text-slate-500",
                                span { class: "text-slate-400", "use " }
                                span { class: "text-slate-500", "lang" }
                                span { class: "text-slate-400", "::" }
                                span { class: "text-slate-400", "{{" }
                                span { class: "text-slate-600",
                                    {languages.join(", ")}
                                }
                                span { class: "text-slate-400", "}};" }
                            }

                            section { class: "mt-6 grid gap-4 md:gap-5 lg:grid-cols-2",
                                for (index, project) in PROJECTS.iter().enumerate() {
                                    a {
                                        href: "{project.url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "group rounded-lg border border-slate-200 bg-white p-4 transition-colors duration-200 hover:border-slate-400 sm:p-5",
                                        div { class: "flex items-start justify-between gap-4",
                                            div { class: "flex min-w-0 items-start gap-3",
                                                div { class: "mt-0.5 flex h-9 w-9 shrink-0 items-center justify-center rounded border border-slate-200 bg-slate-50 text-slate-600",
                                                    svg {
                                                        width: "1em",
                                                        height: "1em",
                                                        view_box: "0 0 24 24",
                                                        fill: "currentColor",
                                                        path { d: "M12 2C6.48 2 2 6.48 2 12c0 4.42 2.87 8.17 6.84 9.5.5.08.66-.23.66-.5v-1.69c-2.77.6-3.36-1.34-3.36-1.34-.46-1.16-1.11-1.47-1.11-1.47-.91-.62.07-.6.07-.6 1 .07 1.53 1.03 1.53 1.03.87 1.52 2.34 1.07 2.91.83.09-.65.35-1.09.63-1.34-2.22-.25-4.55-1.11-4.55-4.92 0-1.11.38-2 1.03-2.71-.1-.25-.45-1.29.1-2.64 0 0 .84-.27 2.75 1.02.79-.22 1.65-.33 2.5-.33.85 0 1.71.11 2.5.33 1.91-1.29 2.75-1.02 2.75-1.02.55 1.35.2 2.39.1 2.64.65.71 1.03 1.6 1.03 2.71 0 3.82-2.34 4.66-4.57 4.91.36.31.69.92.69 1.85V21c0 .27.16.59.67.5C19.14 20.16 22 16.42 22 12A10 10 0 0 0 12 2Z" }
                                                    }
                                                }
                                                div { class: "min-w-0",
                                                    p { class: "text-xs text-slate-400",
                                                        "{project.language} · "
                                                        span { class: "inline-flex items-center gap-0.5",
                                                        svg {
                                                            width: "12",
                                                            height: "12",
                                                            view_box: "0 0 24 24",
                                                            fill: "currentColor",
                                                            class: "text-amber-400",
                                                            path { d: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" }
                                                        }
                                                        "{project_stars(project, repo_stars)}"
                                                    }
                                                }
                                            }
                                            }
                                            span { class: "text-xs text-slate-300", "#{index + 1}" }
                                        }

                                        h2 { class: "mt-3 text-lg font-semibold leading-tight text-slate-900",
                                            "{project.name}"
                                        }
                                        p { class: "mt-2 text-sm leading-relaxed text-slate-600",
                                            "{project.description}"
                                        }

                                        if !project.topics.is_empty() {
                                            p { class: "mt-3 text-xs text-slate-400",
                                                {project.topics.iter().map(|t| format!("#{t}")).collect::<Vec<_>>().join(" ")}
                                            }
                                        }

                                        div { class: "mt-3 inline-flex items-center gap-1 text-xs text-slate-500 transition-colors duration-200 group-hover:text-slate-700",
                                            "open project"
                                            span { ">" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None => rsx! {},
            }
        }
    }
}

fn project_stars(project: &OssProject, repo_stars: Option<&BTreeMap<String, u32>>) -> u32 {
    repo_stars
        .and_then(|stars| stars.get(project.github_repo))
        .copied()
        .unwrap_or(project.stars)
}
