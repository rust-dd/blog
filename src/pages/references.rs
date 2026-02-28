use dioxus::prelude::*;

use crate::{components::loader, seo, ssr::api::select_references};

#[component]
pub fn Component() -> Element {
    let references = use_server_future(select_references)?;
    let title = "Rust Portfolio Projects | Rust-DD";
    let description = "Rust portfolio showcasing high-performance applications in data, AI, and systems engineering.";
    let canonical = seo::absolute_url("/references");

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
            fallback: |_| rsx! { loader::Inline { message: "Loading references...".to_string() } },
            div { class: "container py-12 px-4 mx-auto",
                div { class: "absolute inset-0 overflow-hidden pointer-events-none",
                    div { class: "absolute top-20 right-20 w-72 h-72 bg-[#ffef5c]/3 rounded-full blur-3xl" }
                    div { class: "absolute bottom-20 left-20 w-72 h-72 bg-[#ffef5c]/3 rounded-full blur-3xl" }
                    div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-[#ffef5c]/2 rounded-full blur-3xl" }
                }

                section { id: "about", class: "text-center mb-16 relative",
                    div { class: "inline-flex items-center gap-3 mb-8",
                        div { class: "w-8 h-px bg-[#ffef5c]" }
                        span { class: "text-[#ffef5c] font-mono text-sm tracking-widest", "RUST PORTFOLIO" }
                        div { class: "w-8 h-px bg-[#ffef5c]" }
                    }
                    h1 { class: "text-7xl lg:text-9xl font-black text-white mb-6 leading-none",
                        span { "PROJECT" }
                        br {}
                        span { class: "text-[#ffef5c]", "SHOWCASE" }
                    }
                    p { class: "text-xl text-gray-300 max-w-2xl mx-auto leading-relaxed", "High-performance Rust applications spanning big data processing, AI/ML systems, and real-time streaming solutions." }
                    if let Some(Ok(items)) = references.read().as_ref() {
                        div { class: "flex items-center justify-center gap-8 mt-8",
                            StatCard { value: items.len().to_string(), label: "Projects" }
                            div { class: "w-px h-8 bg-gray-600" }
                            StatCard { value: "100%".to_string(), label: "Rust" }
                            div { class: "w-px h-8 bg-gray-600" }
                            StatCard { value: "2026".to_string(), label: "Latest" }
                        }
                    }
                }

                section { id: "projects", class: "grid lg:grid-cols-2 gap-8 max-w-7xl mx-auto relative",
                    if let Some(result) = references.read().as_ref() {
                        match result {
                            Ok(items) => rsx! {
                                for (index, reference) in items.iter().enumerate() {
                                    article { class: "group relative bg-gradient-to-br from-white/8 to-white/4 rounded-2xl border border-white/10 hover:border-[#ffef5c]/30 transition-all duration-500 overflow-hidden",
                                        div { class: "absolute top-4 right-4 z-10",
                                            span { class: "text-6xl font-black text-[#ffef5c]/10 group-hover:text-[#ffef5c]/20 transition-colors duration-500",
                                                {format!("{:02}", index + 1)}
                                            }
                                        }

                                        div { class: "p-8 border-b border-white/10 relative z-20",
                                            div { class: "flex items-start justify-between mb-4",
                                                div { class: "flex items-center gap-4",
                                                    div { class: "w-14 h-14 bg-[#ffef5c]/10 rounded-xl flex items-center justify-center border border-[#ffef5c]/20 group-hover:bg-[#ffef5c]/20 transition-colors duration-300",
                                                        svg { width: "1.5em", height: "1.5em", view_box: "0 0 24 24", fill: "currentColor", class: "text-[#ffef5c]",
                                                            path { d: "{icon_path(reference.icon.as_deref())}" }
                                                        }
                                                    }
                                                    div {
                                                        div { class: "text-sm text-[#ffef5c] font-mono mb-1", "{reference.year.clone().unwrap_or_default()}" }
                                                        div { class: "text-sm text-gray-400", "{reference.category.clone().unwrap_or_default()}" }
                                                    }
                                                }
                                                if !reference.url.is_empty() {
                                                    a {
                                                        href: "{reference.url}",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        class: "p-2 rounded-lg bg-white/5 hover:bg-[#ffef5c]/10 transition-colors duration-300",
                                                        svg { width: "1.25em", height: "1.25em", view_box: "0 0 24 24", fill: "currentColor", class: "text-gray-400 hover:text-[#ffef5c]",
                                                            path { d: "M14 3h7v7h-2V6.41l-9.29 9.3-1.42-1.42 9.3-9.29H14V3ZM5 5h6v2H7v10h10v-4h2v6H5V5Z" }
                                                        }
                                                    }
                                                }
                                            }
                                            h3 { class: "text-xl font-bold text-white mb-3 group-hover:text-[#ffef5c] transition-colors duration-300 leading-tight pr-16", "{reference.title}" }
                                            p { class: "text-gray-300 leading-relaxed text-sm", "{reference.description}" }
                                        }

                                        div { class: "p-8 relative z-20",
                                            div { class: "mb-6",
                                                h4 { class: "text-[#ffef5c] font-semibold text-xs uppercase tracking-wider mb-3 flex items-center gap-2",
                                                    div { class: "w-1 h-1 bg-[#ffef5c] rounded-full" }
                                                    span { "Technologies" }
                                                }
                                                div { class: "flex flex-wrap gap-2",
                                                    for (idx, tech) in reference.tech_stack.iter().enumerate() {
                                                        TechTag {
                                                            tech: tech.clone(),
                                                            percent: reference.teck_stack_percentage.get(idx).copied().unwrap_or(0)
                                                        }
                                                    }
                                                }
                                            }
                                            div {
                                                h4 { class: "text-gray-400 font-semibold text-xs uppercase tracking-wider mb-3 flex items-center gap-2",
                                                    div { class: "w-1 h-1 bg-gray-400 rounded-full" }
                                                    span { "Tags" }
                                                }
                                                div { class: "flex flex-wrap gap-1.5",
                                                    for tag in reference.tags.iter() {
                                                        span { class: "px-2 py-1 text-xs bg-white/5 text-gray-400 rounded border border-white/10 hover:border-[#ffef5c]/20 hover:text-[#ffef5c] transition-colors duration-300",
                                                            "#{tag}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "absolute inset-0 bg-gradient-to-br from-[#ffef5c]/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none" }
                                        div { class: "absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-[#ffef5c] to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500" }
                                    }
                                }
                            },
                            Err(err) => rsx! { p { class: "text-red-300", "Failed to load references: {err}" } },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(value: String, label: &'static str) -> Element {
    rsx! {
        div { class: "text-center",
            div { class: "text-2xl font-bold text-[#ffef5c]", "{value}" }
            div { class: "text-sm text-gray-400", "{label}" }
        }
    }
}

#[component]
fn TechTag(tech: String, percent: u8) -> Element {
    let class_name = if percent >= 80 {
        "px-3 py-1.5 rounded-lg text-xs font-medium transition-all duration-300 bg-[#ffef5c]/15 text-[#ffef5c] border border-[#ffef5c]/25"
    } else if percent >= 60 {
        "px-3 py-1.5 rounded-lg text-xs font-medium transition-all duration-300 bg-blue-500/15 text-blue-300 border border-blue-500/25"
    } else {
        "px-3 py-1.5 rounded-lg text-xs font-medium transition-all duration-300 bg-white/8 text-gray-300 border border-white/15"
    };

    rsx! {
        span { class: "{class_name}", "{tech}" }
    }
}

fn icon_path(icon: Option<&str>) -> &'static str {
    match icon {
        Some("database") => "M12 2c-5.52 0-10 1.79-10 4v12c0 2.21 4.48 4 10 4s10-1.79 10-4V6c0-2.21-4.48-4-10-4Zm0 2c4.42 0 8 .9 8 2s-3.58 2-8 2-8-.9-8-2 3.58-2 8-2Zm0 16c-4.42 0-8-.9-8-2v-2c1.83 1.19 4.86 2 8 2s6.17-.81 8-2v2c0 1.1-3.58 2-8 2Z",
        Some("iot") => "M11 2h2v3h-2V2Zm0 17h2v3h-2v-3Zm8-8h3v2h-3v-2ZM2 11h3v2H2v-2Zm14.95-6.54 1.41 1.41-2.12 2.12-1.41-1.41 2.12-2.12ZM6.76 14.66l1.41 1.41-2.12 2.12-1.41-1.41 2.12-2.12ZM18.36 17.19l-1.41 1.41-2.12-2.12 1.41-1.41 2.12 2.12ZM8.17 7l-1.41 1.41L4.64 6.29l1.41-1.41L8.17 7Z",
        Some("ai") => "M12 3 2 9l10 6 8-4.8V17h2V9L12 3Zm0 9.64L6.24 9.2 12 5.76l5.76 3.44L12 12.64ZM6 17v2h12v-2H6Z",
        Some("math") => "M5 5h4v2H7v3h2v2H7v5H5V5Zm14 0v12h-2v-5h-2v-2h2V7h-2V5h4ZM9.5 19h5v-2h-5v2Z",
        Some("system") => "M3 3h8v8H3V3Zm10 0h8v5h-8V3ZM3 13h5v8H3v-8Zm7 3h11v5H10v-5Z",
        _ => "M12 2a10 10 0 1 0 10 10A10.011 10.011 0 0 0 12 2Zm7.93 9h-3.01a15.58 15.58 0 0 0-1.18-5.03A8.01 8.01 0 0 1 19.93 11ZM12 4c1.1 0 2.72 2.08 3.03 7H8.97C9.28 6.08 10.9 4 12 4Zm-3.74 1.97A15.58 15.58 0 0 0 7.08 11H4.07a8.01 8.01 0 0 1 4.19-5.03ZM4.07 13h3.01a15.58 15.58 0 0 0 1.18 5.03A8.01 8.01 0 0 1 4.07 13ZM12 20c-1.1 0-2.72-2.08-3.03-7h6.06c-.31 4.92-1.93 7-3.03 7Zm3.74-1.97A15.58 15.58 0 0 0 16.92 13h3.01a8.01 8.01 0 0 1-4.19 5.03Z",
    }
}
