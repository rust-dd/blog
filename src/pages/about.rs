use dioxus::prelude::*;

use crate::{
    app::Route,
    components::shell::{SectionHeading, SideGroup, Sidebar},
    pages::opensource::{project_stars, PROJECTS},
    search::SearchQuery,
    seo,
    ssr::{
        api::{select_posts, select_repo_stars},
        types::Post,
    },
};

#[derive(PartialEq)]
struct Entry {
    years: &'static str,
    title: &'static str,
    place: &'static str,
    note: &'static str,
}

struct Paper {
    title: &'static str,
    venue: &'static str,
    url: &'static str,
}

const AUTHOR_NAME: &str = "Daniel Boros";
const PORTRAIT_PATH: &str = "/daniel-boros.jpg";
/// `None` hides the CV button.
const CV_PATH: Option<&str> = Some("/cv.pdf");

const MAINTAINED: &[&str] = &[
    "stochastic-rs",
    "tako",
    "rsql",
    "candding",
    "iTransformer",
    "react-native-qdrant-edge",
    "async-safe-defer",
    "react-native-scc",
    "ito",
];

const EXPERIENCE: &[Entry] = &[
    Entry {
        years: "2025..",
        title: "Senior Rust Engineer",
        place: "Qdrant",
        note: "Core engineer on scalable cloud inference and the core product.",
    },
    Entry {
        years: "2025..",
        title: "Core Rust Engineer, part-time",
        place: "AlgoFusion",
        note: "Quant lead; designs and builds the trading engine.",
    },
    Entry {
        years: "2024..",
        title: "Co-owner",
        place: "Rust-DD",
        note: "Founded and runs the Rust-DD open-source community and this blog with Daniel Zelei.",
    },
    Entry {
        years: "2022..2025",
        title: "Senior Software Engineer",
        place: "Kadasolutions",
        note: "Backend, AI and data processing in Rust: Tokio, Axum, Candle, ONNX, Rayon and GeoRust.",
    },
    Entry {
        years: "2021..2025",
        title: "Researcher",
        place: "AI Research Group, Eötvös Loránd University",
        note: "Deep learning and parameter estimation of fractional processes; maintaining stochastic-rs.",
    },
    Entry {
        years: "2018..",
        title: "Chief Technology Officer",
        place: "GymPiper",
        note: "Leads mobile and web development on Rust, TypeScript and Google Cloud.",
    },
    Entry {
        years: "2012..2018",
        title: "Principal CFO",
        place: "GetPro",
        note: "Finance and accounting; taught advanced mathematics and mechanics to engineering students.",
    },
];

const EDUCATION: &[Entry] = &[
    Entry {
        years: "2022..",
        title: "PhD, Computational and Applied Mathematics",
        place: "Eötvös Loránd University",
        note: "",
    },
    Entry {
        years: "2019..2022",
        title: "MSc, Actuarial and Financial Mathematics",
        place: "Corvinus University of Budapest",
        note: "",
    },
    Entry {
        years: "2009..2017",
        title: "BSc, Civil Engineering",
        place: "Budapest University of Technology and Economics",
        note: "",
    },
];

const PAPERS: &[Paper] = &[
    Paper {
        title: "A Fractional Process with Jumps for Modeling Karstic Spring Discharge Data",
        venue: "Mathematics 13(18), 2025",
        url: "https://www.mdpi.com/2227-7390/13/18/2928",
    },
    Paper {
        title: "Deep learning the Hurst parameter of linear fractional processes and assessing its reliability",
        venue: "Quality and Reliability Engineering International, 2024",
        url: "https://doi.org/10.1002/qre.3641",
    },
    Paper {
        title: "Parameter Estimation of Long Memory Stochastic Processes with Deep Neural Networks",
        venue: "arXiv:2410.03776, 2024, with Bálint Csanády et al.",
        url: "https://arxiv.org/abs/2410.03776",
    },
];

const AWARD: &str = "Best Presentation Award at ICMFE 2023 in New York, for “Fast Estimation of Fractional Process Parameters in Rough Financial Models Using Artificial Intelligence”.";

const TOOLBOX: &[(&str, &str)] = &[
    ("Languages", "Rust, TypeScript, Python, R"),
    ("Backend", "Tokio, Axum, Actix, Tower, Hono"),
    ("Frontend", "React, React Native, Dioxus, Leptos, Tauri"),
    ("AI and ML", "Candle, Mistral.rs, tch-rs, OpenCV, TensorFlow, Keras"),
    ("Data", "Polars, Rayon, ndarray, GeoRust, pandas, scikit-learn"),
    ("Infrastructure", "Google Cloud, Docker, PostgreSQL, GraphQL"),
    ("Spoken", "Hungarian, English, German"),
];

const TRAITS: &[&str] = &["Maintainer", "Author", "Researcher", "Experience", "Education", "Toolbox"];

#[component]
pub fn Component() -> Element {
    let posts = use_server_future(select_posts)?;
    let stars = use_server_future(select_repo_stars)?;

    let own_posts: Vec<Post> = posts
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|posts| {
            posts
                .iter()
                .filter(|post| post.author.name == AUTHOR_NAME)
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let stars = stars.read().as_ref().and_then(|result| result.as_ref().ok()).cloned();
    let crates: Vec<_> = MAINTAINED
        .iter()
        .filter_map(|name| PROJECTS.iter().find(|project| project.name == *name))
        .map(|project| (project, project_stars(project, stars.as_ref())))
        .collect();

    rsx! {
        seo::PageMeta {
            title: seo::page_title("Daniel Boros: Rust engineer and researcher"),
            description: "Daniel Boros is a Senior Rust Engineer at Qdrant and a PhD researcher in fractional stochastic processes who maintains stochastic-rs and Tako.",
            path: "/about",
            og_type: "profile",
        }
        seo::JsonLd { value: seo::person_graph() }

        Sidebar {
            SideGroup { title: "Struct DanielBoros",
                li { a { href: "#fields", class: "side-link", "Fields" } }
                li { a { href: "#implementations", class: "side-link", "Implementations" } }
                li { a { href: "#trait-implementations", class: "side-link", "Trait Implementations" } }
            }
            SideGroup { title: "Trait Implementations",
                for name in TRAITS.iter() {
                    li { a { href: "#impl-{name.to_lowercase()}", class: "side-link side-code text-mod", "{name}" } }
                }
            }
            SideGroup { title: "In rust_dd::authors",
                li {
                    a {
                        href: "#main",
                        aria_current: "page",
                        class: "side-link side-code side-current text-author",
                        "DanielBoros"
                    }
                }
                li {
                    a {
                        href: "https://github.com/zeldan",
                        rel: "noopener noreferrer",
                        target: "_blank",
                        class: "side-link side-code text-author",
                        "DanielZelei"
                    }
                }
            }
        }

        main { id: "main", class: "shell-main",
            p { class: "doc-path",
                Link { to: Route::Home { query: SearchQuery::default() }, class: "text-mod", "rust_dd" }
                "::"
                span { class: "text-mod", "authors" }
            }
            div { class: "doc-title",
                h1 { class: "doc-h1",
                    "Struct "
                    span { class: "text-author", "DanielBoros" }
                }
                a {
                    href: "https://github.com/dancixx",
                    rel: "noopener noreferrer",
                    target: "_blank",
                    class: "doc-source",
                    "Source"
                }
            }
            pre { class: "code-block decl",
                code {
                    span { class: "fm-line",
                        span { class: "sx-storage", "pub struct " }
                        span { class: "text-author", "DanielBoros" }
                        " {{"
                    }
                    span { class: "fm-line",
                        "    "
                        span { class: "sx-storage", "pub " }
                        "role: &"
                        span { class: "sx-storage", "'static " }
                        span { class: "sx-support sx-type", "str" }
                        ","
                    }
                    span { class: "fm-line",
                        "    "
                        span { class: "sx-storage", "pub " }
                        "location: &"
                        span { class: "sx-storage", "'static " }
                        span { class: "sx-support sx-type", "str" }
                        ","
                    }
                    span { class: "fm-line",
                        "    "
                        span { class: "sx-storage", "pub " }
                        "research: ["
                        span { class: "text-author", "Topic" }
                        "; 3],"
                    }
                    span { class: "fm-line", "}}" }
                }
            }

            div { class: "doc-text", style: "margin-top: 26px",
                img {
                    src: PORTRAIT_PATH,
                    alt: "Portrait of Daniel Boros",
                    width: "176",
                    height: "176",
                    class: "portrait",
                }
                p {
                    "I’m a Rust-focused developer and researcher with a passion for combining high-performance systems programming with the theoretical depth of AI and financial mathematics. My background bridges applied software engineering and academic research, from building low-latency systems in Rust to working on advanced models in stochastic analysis."
                }
                p { style: "margin-top: 0.85em",
                    "Whether it’s crafting performant Rust systems, designing scalable infrastructure, or exploring the mathematics behind market behaviour, I thrive at the intersection of theory and real-world impact."
                }
                div { style: "clear: both" }
            }

            SectionHeading { id: "fields", title: "Fields" }
            dl {
                Field { signature: "role: &'static str", doc: "Senior Rust Engineer at Qdrant." }
                Field { signature: "location: &'static str", doc: "Budapest, Hungary." }
                Field {
                    signature: "research: [Topic; 3]",
                    doc: "Fractional stochastic processes, rough path theory and Malliavin calculus, applied to financial modelling, stochastic simulation and AI-driven quantitative methods. All of the implementations somehow end up in Rust.",
                }
            }

            SectionHeading { id: "implementations", title: "Implementations" }
            details { class: "impl", open: true,
                summary {
                    span { class: "sx-storage", "impl " }
                    span { class: "text-author", "DanielBoros" }
                }
                div { class: "impl-body",
                    if let Some(cv) = CV_PATH {
                        p { class: "ident",
                            span { class: "sx-storage", "pub fn " }
                            span { class: "text-post", "cv" }
                            "(&self) -> "
                            span { class: "text-author", "Pdf" }
                        }
                        p { class: "doc-text", style: "margin: 6px 0 0 24px", "The full CV as a PDF." }
                        a {
                            href: cv,
                            class: "button-primary",
                            style: "margin: 12px 0 0 24px",
                            download: "Daniel-Boros-CV.pdf",
                            "Download CV (PDF)"
                        }
                    }
                    p { class: "ident", style: "margin-top: 26px",
                        span { class: "sx-storage", "pub fn " }
                        span { class: "text-post", "contact" }
                        "(&self) -> ["
                        span { class: "text-author", "Link" }
                        "; 3]"
                    }
                    p {
                        class: "doc-text",
                        style: "margin: 6px 0 0 24px; display: flex; flex-wrap: wrap; gap: 4px 20px",
                        a {
                            href: "https://github.com/dancixx",
                            rel: "noopener noreferrer",
                            target: "_blank",
                            class: "text-mod",
                            "GitHub"
                        }
                        a {
                            href: "https://www.linkedin.com/in/daniel-boros-b86a5373/",
                            rel: "noopener noreferrer",
                            target: "_blank",
                            class: "text-mod",
                            "LinkedIn"
                        }
                        a { href: "mailto:dancixx@gmail.com", class: "text-mod", "dancixx@gmail.com" }
                    }
                }
            }

            SectionHeading { id: "trait-implementations", title: "Trait Implementations" }
            Impl { name: "Maintainer",
                div { class: "table-scroll",
                    table { class: "data-table",
                        thead {
                            tr {
                                th { scope: "col", "Crate" }
                                th { scope: "col", "What it does" }
                                th { scope: "col", class: "num", "Stars" }
                            }
                        }
                        tbody {
                            for (project, count) in crates {
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
                                    td { "{project.description}" }
                                    td { class: "num", "{count}" }
                                }
                            }
                        }
                    }
                }
            }
            Impl { name: "Author",
                p { class: "doc-text", "{own_posts.len()} posts on rust-dd since August 2024." }
                ul { class: "post-list",
                    for post in own_posts.iter().take(3) {
                        li { class: "post-row",
                            time { datetime: "{post.created_at}", "{post.published_on()}" }
                            Link {
                                to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                class: "post-row-title",
                                "{post.title}"
                            }
                        }
                    }
                }
                p { style: "margin-top: 10px; font-size: 15px",
                    Link { to: Route::Home { query: SearchQuery::default() }, class: "text-mod", "All posts" }
                }
            }
            Impl { name: "Researcher",
                ul { class: "post-list",
                    for paper in PAPERS.iter() {
                        li { class: "post-row",
                            a {
                                href: "{paper.url}",
                                rel: "noopener noreferrer",
                                target: "_blank",
                                class: "post-row-title",
                                "{paper.title}"
                            }
                            span { class: "post-row-topic", "{paper.venue}" }
                        }
                    }
                }
                p { class: "doc-text", style: "margin-top: 12px; font-size: 17px", "{AWARD}" }
            }
            Impl { name: "Experience", Timeline { entries: EXPERIENCE } }
            Impl { name: "Education", Timeline { entries: EDUCATION } }
            Impl { name: "Toolbox",
                dl { class: "item-table",
                    for (label, value) in TOOLBOX.iter() {
                        div { class: "item-row",
                            dt { class: "text-muted", style: "font-size: 15px", "{label}" }
                            dd { "{value}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Field(signature: &'static str, doc: &'static str) -> Element {
    rsx! {
        dt { class: "ident", style: "margin-top: 16px; font-size: 16px", "{signature}" }
        dd { class: "doc-text", style: "margin: 6px 0 0 24px; max-width: 40em", "{doc}" }
    }
}

#[component]
fn Impl(name: &'static str, children: Element) -> Element {
    rsx! {
        details { id: "impl-{name.to_lowercase()}", class: "impl", open: true,
            summary {
                span { class: "sx-storage", "impl " }
                span { class: "text-mod", "{name}" }
                " for "
                span { class: "text-author", "DanielBoros" }
            }
            div { class: "impl-body", {children} }
        }
    }
}

#[component]
fn Timeline(entries: &'static [Entry]) -> Element {
    rsx! {
        ol { class: "post-list",
            for entry in entries.iter() {
                li { class: "post-row",
                    span { class: "post-row-topic", style: "flex: 0 0 108px", "{entry.years}" }
                    div { style: "flex: 1 1 380px; min-width: 0",
                        p { class: "text-heading", style: "font-weight: 600",
                            "{entry.title}"
                            span { class: "text-muted", style: "font-weight: 400", ", {entry.place}" }
                        }
                        if !entry.note.is_empty() {
                            p { class: "doc-text", style: "font-size: 16px; margin-top: 2px", "{entry.note}" }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CV_PATH, MAINTAINED, PORTRAIT_PATH};
    use crate::pages::opensource::PROJECTS;

    #[test]
    fn maintained_crates_are_listed_as_open_source() {
        let missing: Vec<&str> = MAINTAINED
            .iter()
            .copied()
            .filter(|name| !PROJECTS.iter().any(|project| project.name == *name))
            .collect();

        assert!(missing.is_empty(), "missing from the open source list: {missing:?}");
    }

    #[test]
    fn linked_files_are_published() {
        let published = |path: &str| std::path::Path::new("public").join(path.trim_start_matches('/')).exists();

        assert!(published(PORTRAIT_PATH));
        assert!(CV_PATH.map_or(true, published));
    }
}
