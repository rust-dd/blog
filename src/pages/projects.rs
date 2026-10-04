use dioxus::prelude::*;

use crate::{
    app::Route,
    components::shell::{SideGroup, Sidebar, CONTENT_ID},
    search::SearchQuery,
    seo,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Project {
    pub name: &'static str,
    pub kind: &'static str,
    pub description: &'static str,
    pub url: &'static str,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "stochastic-rs",
        kind: "library and docs",
        description: "Open-source quantitative finance for Rust and Python: 132 stochastic processes, option pricing, Heston and SABR calibration, volatility surfaces and fixed income.",
        url: "https://stochastic.rust-dd.com",
    },
    Project {
        name: "tako",
        kind: "web framework",
        description: "Multi-transport Rust web framework: HTTP/1.1, HTTP/2, HTTP/3, WebTransport, WebSocket, SSE, gRPC, TCP, UDP and Unix sockets behind one router.",
        url: "https://tako.rust-dd.com",
    },
    Project {
        name: "candding",
        kind: "library",
        description: "Dense, sparse, multi-vector and reranking models in Rust on candle, with no ONNX runtime, each checked against its reference implementation.",
        url: "https://candding.rust-dd.com",
    },
    Project {
        name: "rsql",
        kind: "database client",
        description: "Fast PostgreSQL client built with Rust, Tauri and React. Query data, inspect schema, run EXPLAIN and stay responsive on large result sets.",
        url: "https://rsql.rust-dd.com",
    },
    Project {
        name: "stochasticlab",
        kind: "platform",
        description: "Quantitative tools built on stochastic calculus: portfolio optimization, volatility modeling, option pricing and risk analysis.",
        url: "https://stochasticlab.cloud",
    },
    Project {
        name: "shrtn.ink",
        kind: "service",
        description: "Free URL shortener with QR codes, click analytics, a REST API, an MCP server and webhooks.",
        url: "https://shrtn.ink",
    },
    Project {
        name: "tryrust.org",
        kind: "education",
        description: "Interactive Rust tutorial and playground that runs directly in the browser.",
        url: "https://tryrust.org",
    },
    Project {
        name: "doom.rust-dd",
        kind: "experiment",
        description: "DOOM in the browser, rebuilt with Rust and Bevy on Freedoom.",
        url: "https://doom.rust-dd.com",
    },
    Project {
        name: "react-native-scc",
        kind: "library",
        description: "Rust-powered persistent key-value storage for React Native and Expo, built as a drop-in MMKV alternative.",
        url: "https://github.com/rust-dd/react-native-scc",
    },
    Project {
        name: "react-native-qdrant-edge",
        kind: "library",
        description: "Embedded vector search for React Native: the Qdrant engine running in-process on iOS and Android.",
        url: "https://github.com/rust-dd/react-native-qdrant-edge",
    },
    Project {
        name: "ito",
        kind: "terminal app",
        description: "Terminal UI to browse, configure and plot every stochastic process in stochastic-rs.",
        url: "https://github.com/rust-dd/ito",
    },
];

#[component]
pub fn Component() -> Element {
    rsx! {
        seo::PageMeta {
            title: seo::page_title("Rust Apps and Developer Tools"),
            description: "Apps, sites and tools we build with Rust: stochastic-rs, the Tako web framework, candding embeddings, the rsql PostgreSQL client and the tryrust.org playground.",
            path: "/projects",
        }
        Sidebar {
            SideGroup { title: "Projects",
                for project in PROJECTS.iter() {
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
            }
            p { class: "doc-lead", "Live tools, libraries with their own sites, experiments and product work from Rust-DD." }
            dl { class: "item-table", style: "margin-top: 24px",
                for project in PROJECTS.iter() {
                    div { id: "{project.name}", class: "item-row",
                        dt {
                            a {
                                href: "{project.url}",
                                rel: "noopener noreferrer",
                                target: "_blank",
                                class: "ident text-mod",
                                "{project.name}"
                            }
                            span { class: "item-kind", "{project.kind}" }
                        }
                        dd { "{project.description}" }
                    }
                }
            }
        }
    }
}
