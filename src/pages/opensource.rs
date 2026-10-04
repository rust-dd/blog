use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::{
    app::Route,
    components::shell::{SectionHeading, SideGroup, Sidebar},
    seo,
    ssr::api::select_repo_stars,
};

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
        name: "stochastic-rs",
        description: "High-performance quantitative finance in Rust and Python: 130+ stochastic processes, option pricing, calibration, fixed income, risk and copulas, with SIMD and GPU acceleration.",
        url: "https://github.com/rust-dd/stochastic-rs",
        github_repo: "rust-dd/stochastic-rs",
        stars: 190,
        language: "Rust",
        topics: &["quant", "stochastic-processes", "option-pricing", "rough-volatility", "cuda"],
    },
    OssProject {
        name: "tako",
        description: "Multi-transport Rust web framework: HTTP/1.1, HTTP/2, HTTP/3, WebSocket, SSE, gRPC, TCP/UDP and Unix sockets behind one router, on Tokio or Compio.",
        url: "https://github.com/rust-dd/tako",
        github_repo: "rust-dd/tako",
        stars: 164,
        language: "Rust",
        topics: &["async", "http3", "grpc", "websocket", "io-uring"],
    },
    OssProject {
        name: "rsql",
        description: "Fast PostgreSQL client built with Rust, Tauri and React for querying data, running EXPLAIN and exploring large result sets.",
        url: "https://github.com/rust-dd/rsql",
        github_repo: "rust-dd/rsql",
        stars: 76,
        language: "TypeScript",
        topics: &["postgresql", "tauri", "react"],
    },
    OssProject {
        name: "rust-axum-async-graphql-postgres-redis-starter",
        description: "Starter template using Rust with Axum, Async-GraphQL, PostgreSQL and Redis for building high-performance web APIs.",
        url: "https://github.com/rust-dd/rust-axum-async-graphql-postgres-redis-starter",
        github_repo: "rust-dd/rust-axum-async-graphql-postgres-redis-starter",
        stars: 45,
        language: "Rust",
        topics: &["axum", "graphql", "postgres", "redis"],
    },
    OssProject {
        name: "embedded-dht-rs",
        description: "A Rust library with full support for DHT11, DHT22 and DHT20 (AHT20) temperature and humidity sensors.",
        url: "https://github.com/rust-dd/embedded-dht-rs",
        github_repo: "rust-dd/embedded-dht-rs",
        stars: 39,
        language: "Rust",
        topics: &["dht11", "dht22", "aht20", "esp32", "embedded"],
    },
    OssProject {
        name: "aoc-2024",
        description: "Advent of Code 2024 puzzles, solved in Rust.",
        url: "https://github.com/rust-dd/aoc-2024",
        github_repo: "rust-dd/aoc-2024",
        stars: 24,
        language: "Rust",
        topics: &["advent-of-code"],
    },
    OssProject {
        name: "iTransformer",
        description: "An iTransformer implementation in Rust for time-series forecasting.",
        url: "https://github.com/rust-dd/iTransformer",
        github_repo: "rust-dd/iTransformer",
        stars: 22,
        language: "Rust",
        topics: &["ai", "transformers", "mathematics"],
    },
    OssProject {
        name: "blog",
        description: "Blog engine written in Rust, powered by Dioxus and SurrealDB.",
        url: "https://github.com/rust-dd/blog",
        github_repo: "rust-dd/blog",
        stars: 18,
        language: "Rust",
        topics: &["blog", "dioxus", "surrealdb"],
    },
    OssProject {
        name: "google-calendar-cli",
        description: "Google Calendar CLI written in Rust.",
        url: "https://github.com/rust-dd/google-calendar-cli",
        github_repo: "rust-dd/google-calendar-cli",
        stars: 16,
        language: "Rust",
        topics: &["cli", "google-calendar"],
    },
    OssProject {
        name: "probability-benchmark",
        description: "Scientific computing benchmark: Rust vs Zig vs C on Ornstein–Uhlenbeck processes.",
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
        stars: 10,
        language: "Rust",
        topics: &["axum", "leptos", "tutorial"],
    },
    OssProject {
        name: "react-native-qdrant-edge",
        description: "Embedded vector search for React Native: the Qdrant engine running in-process on iOS and Android.",
        url: "https://github.com/rust-dd/react-native-qdrant-edge",
        github_repo: "rust-dd/react-native-qdrant-edge",
        stars: 10,
        language: "TypeScript",
        topics: &["qdrant", "react-native", "vector-search"],
    },
    OssProject {
        name: "async-safe-defer",
        description: "Minimal async- and sync-capable defer crate.",
        url: "https://github.com/rust-dd/async-safe-defer",
        github_repo: "rust-dd/async-safe-defer",
        stars: 9,
        language: "Rust",
        topics: &["async", "defer", "embedded"],
    },
    OssProject {
        name: "react-native-scc",
        description: "Rust-powered persistent key-value storage for React Native and Expo via Nitro Modules: a lock-free hash map built as a drop-in MMKV alternative.",
        url: "https://github.com/rust-dd/react-native-scc",
        github_repo: "rust-dd/react-native-scc",
        stars: 4,
        language: "Rust",
        topics: &["react-native", "nitro-modules", "key-value-store"],
    },
    OssProject {
        name: "async-rs",
        description: "A minimal, educational async runtime in Rust with a lightweight executor and task system.",
        url: "https://github.com/rust-dd/async-rs",
        github_repo: "rust-dd/async-rs",
        stars: 3,
        language: "Rust",
        topics: &["async", "runtime"],
    },
    OssProject {
        name: "xor-neural-network",
        description: "A minimal neural network that learns XOR from scratch with only the standard library and rand.",
        url: "https://github.com/rust-dd/xor-neural-network",
        github_repo: "rust-dd/xor-neural-network",
        stars: 2,
        language: "Rust",
        topics: &["neural-network", "machine-learning"],
    },
    OssProject {
        name: "ito",
        description: "Terminal UI to browse, configure and plot every stochastic process in stochastic-rs: Monte-Carlo paths on the CPU, in f64.",
        url: "https://github.com/rust-dd/ito",
        github_repo: "rust-dd/ito",
        stars: 2,
        language: "Rust",
        topics: &["tui", "stochastic-processes", "quant"],
    },
    OssProject {
        name: "candding",
        description: "Pure candle embeddings for Rust: dense, sparse, multi-vector and reranking models with no ONNX runtime, each checked against its reference implementation.",
        url: "https://github.com/rust-dd/candding",
        github_repo: "rust-dd/candding",
        stars: 0,
        language: "Rust",
        topics: &["candle", "embeddings", "machine-learning"],
    },
    OssProject {
        name: "react-state-rs",
        description: "Minimal state management for React applications, built with Rust and compiled to WebAssembly.",
        url: "https://github.com/rust-dd/react-state-rs",
        github_repo: "rust-dd/react-state-rs",
        stars: 0,
        language: "Rust",
        topics: &["wasm", "react"],
    },
    OssProject {
        name: "impl-new-derive",
        description: "A derive macro that generates struct constructors: public fields from arguments, private fields from their defaults, generic or not.",
        url: "https://github.com/rust-dd/impl-new-derive",
        github_repo: "rust-dd/impl-new-derive",
        stars: 0,
        language: "Rust",
        topics: &["macro", "derive"],
    },
];

#[component]
pub fn Component() -> Element {
    let stars = use_server_future(select_repo_stars)?;
    let repo_stars = stars.read().as_ref().and_then(|result| result.as_ref().ok()).cloned();

    let mut sorted: Vec<&OssProject> = PROJECTS.iter().collect();
    sorted.sort_by(|a, b| {
        project_stars(b, repo_stars.as_ref())
            .cmp(&project_stars(a, repo_stars.as_ref()))
            .then_with(|| a.name.cmp(b.name))
    });
    let total_stars: u32 = PROJECTS
        .iter()
        .map(|project| project_stars(project, repo_stars.as_ref()))
        .sum();
    let mut languages: BTreeMap<&str, usize> = BTreeMap::new();
    for project in PROJECTS.iter() {
        *languages.entry(project.language).or_insert(0) += 1;
    }

    rsx! {
        seo::PageMeta {
            title: seo::page_title("Open-Source Rust Crates and Tools"),
            description: "Open-source Rust from Rust-DD: stochastic-rs for quant finance, the Tako web framework, candding embeddings, embedded sensor drivers, CLI tools and starter templates.",
            path: "/opensource",
        }
        Sidebar {
            SideGroup { title: "Sections",
                li { a { href: "#crates", class: "side-link", "Crates" } }
            }
            SideGroup { title: "Languages",
                for (language, count) in languages.iter() {
                    li { span { class: "side-link", "{language} ({count})" } }
                }
            }
        }
        main { id: "main", class: "shell-main",
            p { class: "doc-path",
                Link { to: Route::Home { q: String::new() }, class: "text-mod", "rust_dd" }
                "::"
                span { class: "text-mod", "open_source" }
            }
            div { class: "doc-title",
                h1 { class: "doc-h1",
                    "Module "
                    span { class: "text-mod", "open_source" }
                }
                a {
                    href: "https://github.com/rust-dd",
                    rel: "noopener noreferrer",
                    target: "_blank",
                    class: "doc-source",
                    "GitHub"
                }
            }
            p { class: "doc-lead",
                "Libraries, frameworks and tools we build and maintain in the open: {PROJECTS.len()} repositories and {total_stars} stars."
            }
            SectionHeading { id: "crates", title: "Crates" }
            div { class: "table-scroll",
                table { class: "data-table",
                    thead {
                        tr {
                            th { scope: "col", "Crate" }
                            th { scope: "col", "What it does" }
                            th { scope: "col", "Language" }
                            th { scope: "col", class: "num", "Stars" }
                        }
                    }
                    tbody {
                        for project in sorted {
                            tr {
                                td {
                                    a {
                                        href: "{project.url}",
                                        rel: "noopener noreferrer",
                                        target: "_blank",
                                        class: "ident text-mod",
                                        "{project.name}"
                                    }
                                }
                                td {
                                    "{project.description}"
                                    if !project.topics.is_empty() {
                                        p { class: "item-tags", {project.topics.join(" · ")} }
                                    }
                                }
                                td { class: "text-muted font-sans", "{project.language}" }
                                td { class: "num", "{project_stars(project, repo_stars.as_ref())}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn project_stars(project: &OssProject, repo_stars: Option<&BTreeMap<String, u32>>) -> u32 {
    repo_stars
        .and_then(|stars| stars.get(project.github_repo))
        .copied()
        .unwrap_or(project.stars)
}
