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
            div { class: "mx-auto w-full max-w-6xl font-mono",
                section { class: "relative overflow-hidden rounded-3xl border border-slate-200 bg-white/92 p-5 shadow-sm sm:p-7 md:p-10",
                    div { class: "pointer-events-none absolute -top-20 right-0 h-72 w-72 rounded-full bg-slate-300/20 blur-3xl" }
                    p { class: "relative z-10 text-[11px] font-semibold uppercase tracking-[0.2em] text-slate-700", "Portfolio" }
                    h1 { class: "relative z-10 mt-3 text-3xl font-semibold leading-tight text-slate-900 sm:text-4xl md:text-5xl", "Built Systems" }
                    p { class: "relative z-10 mt-3 max-w-3xl text-sm leading-relaxed text-slate-600 md:text-base",
                        "Production Rust systems across data, AI, and performance-heavy backend domains."
                    }
                }

                if let Some(result) = references.read().as_ref() {
                    match result {
                        Ok(items) => {
                            let latest_year = items
                                .iter()
                                .filter_map(|item| item.year.clone())
                                .find(|year| !year.trim().is_empty())
                                .unwrap_or_else(|| "n/a".to_string());

                            let mut categories: Vec<String> = items
                                .iter()
                                .filter_map(|item| item.category.clone())
                                .filter(|category| !category.trim().is_empty())
                                .collect();
                            categories.sort_by_key(|category| category.to_lowercase());
                            categories.dedup_by(|a, b| a.eq_ignore_ascii_case(b));

                            rsx! {
                                div { class: "mt-6 grid gap-4 sm:grid-cols-3",
                                    StatCard { value: items.len().to_string(), label: "projects" }
                                    StatCard { value: "rust".to_string(), label: "core stack" }
                                    StatCard { value: latest_year, label: "latest year" }
                                }

                                if !categories.is_empty() {
                                    div { class: "mt-4 rounded-2xl border border-slate-200 bg-white/90 p-4 shadow-sm",
                                        p { class: "text-[11px] font-semibold uppercase tracking-[0.18em] text-slate-500", "categories" }
                                        div { class: "mt-3 flex flex-wrap gap-2",
                                            for category in categories {
                                                span { class: "rounded-full border border-slate-200 bg-slate-50 px-2.5 py-1 text-[11px] text-slate-700",
                                                    "{category}"
                                                }
                                            }
                                        }
                                    }
                                }

                                section { class: "mt-6 grid gap-4 md:gap-5 lg:grid-cols-2",
                                    for (index, reference) in items.iter().enumerate() {
                                        article { class: "group relative overflow-hidden rounded-2xl border border-slate-200 bg-white/90 p-4 shadow-sm transition-all duration-200 hover:border-slate-400 hover:shadow-md sm:p-6",
                                            div { class: "flex items-start justify-between gap-4",
                                                div { class: "flex min-w-0 items-start gap-3",
                                                    div { class: "mt-0.5 flex h-11 w-11 shrink-0 items-center justify-center rounded-lg border border-slate-200 bg-slate-50 text-slate-700",
                                                        svg {
                                                            width: "1.2em",
                                                            height: "1.2em",
                                                            view_box: "0 0 24 24",
                                                            fill: "currentColor",
                                                            path { d: "{icon_path(reference.icon.as_deref())}" }
                                                        }
                                                    }
                                                    div { class: "min-w-0",
                                                        p { class: "text-[11px] font-semibold uppercase tracking-[0.18em] text-slate-500",
                                                            {reference.year.clone().unwrap_or_else(|| "n/a".to_string())}
                                                        }
                                                        p { class: "mt-1 text-xs text-slate-500",
                                                            {reference.category.clone().unwrap_or_else(|| "General".to_string())}
                                                        }
                                                    }
                                                }
                                                span { class: "rounded-full border border-slate-200 bg-slate-50 px-2 py-0.5 text-xs text-slate-500",
                                                    "#{index + 1}"
                                                }
                                            }

                                            h2 { class: "mt-4 text-lg font-semibold leading-tight text-slate-900 sm:text-xl", "{reference.title}" }
                                            p { class: "mt-2 text-sm leading-relaxed text-slate-600", "{reference.description}" }

                                            if !reference.tech_stack.is_empty() {
                                                div { class: "mt-4",
                                                    p { class: "text-[11px] font-semibold uppercase tracking-[0.16em] text-slate-500", "stack" }
                                                    div { class: "mt-2 flex flex-wrap gap-2",
                                                        for (idx, tech) in reference.tech_stack.iter().enumerate() {
                                                            TechTag {
                                                                tech: tech.clone(),
                                                                percent: reference.teck_stack_percentage.get(idx).copied().unwrap_or(0)
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            if !reference.tags.is_empty() {
                                                div { class: "mt-4 flex flex-wrap gap-1.5",
                                                    for tag in reference.tags.iter().take(8) {
                                                        span { class: "rounded-md border border-slate-200 bg-slate-50 px-2 py-1 text-xs text-slate-600",
                                                            "#{tag}"
                                                        }
                                                    }
                                                }
                                            }

                                            if !reference.url.is_empty() {
                                                a {
                                                    href: "{reference.url}",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "mt-5 inline-flex items-center gap-2 rounded-full border border-slate-200 bg-slate-50 px-3 py-1.5 text-xs font-semibold text-slate-700 transition-colors duration-200 hover:bg-slate-100",
                                                    "Open project"
                                                    span { "↗" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => rsx! {
                            p { class: "mt-8 text-red-600", "Failed to load references: {err}" }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(value: String, label: &'static str) -> Element {
    rsx! {
        div { class: "rounded-2xl border border-slate-200 bg-white/90 p-4 shadow-sm",
            p { class: "text-2xl font-semibold text-slate-900", "{value}" }
            p { class: "mt-1 text-[11px] font-semibold uppercase tracking-[0.18em] text-slate-500", "{label}" }
        }
    }
}

#[component]
fn TechTag(tech: String, percent: u8) -> Element {
    let class_name = if percent >= 80 {
        "rounded-lg border border-slate-200 bg-slate-50 px-2.5 py-1 text-xs font-medium text-slate-700"
    } else if percent >= 60 {
        "rounded-lg border border-slate-200 bg-slate-50 px-2.5 py-1 text-xs font-medium text-slate-700"
    } else {
        "rounded-lg border border-slate-200 bg-slate-50 px-2.5 py-1 text-xs font-medium text-slate-600"
    };

    rsx! {
        span { class: "{class_name}", "{tech}" }
    }
}

fn icon_path(icon: Option<&str>) -> &'static str {
    match icon {
        Some("database") => {
            "M12 2c-5.52 0-10 1.79-10 4v12c0 2.21 4.48 4 10 4s10-1.79 10-4V6c0-2.21-4.48-4-10-4Zm0 2c4.42 0 8 .9 8 2s-3.58 2-8 2-8-.9-8-2 3.58-2 8-2Zm0 16c-4.42 0-8-.9-8-2v-2c1.83 1.19 4.86 2 8 2s6.17-.81 8-2v2c0 1.1-3.58 2-8 2Z"
        }
        Some("iot") => {
            "M11 2h2v3h-2V2Zm0 17h2v3h-2v-3Zm8-8h3v2h-3v-2ZM2 11h3v2H2v-2Zm14.95-6.54 1.41 1.41-2.12 2.12-1.41-1.41 2.12-2.12ZM6.76 14.66l1.41 1.41-2.12 2.12-1.41-1.41 2.12-2.12ZM18.36 17.19l-1.41 1.41-2.12-2.12 1.41-1.41 2.12 2.12ZM8.17 7l-1.41 1.41L4.64 6.29l1.41-1.41L8.17 7Z"
        }
        Some("ai") => "M12 3 2 9l10 6 8-4.8V17h2V9L12 3Zm0 9.64L6.24 9.2 12 5.76l5.76 3.44L12 12.64ZM6 17v2h12v-2H6Z",
        Some("math") => {
            "M5 5h4v2H7v3h2v2H7v5H5V5Zm14 0v12h-2v-5h-2v-2h2V7h-2V5h4ZM9.5 19h5v-2h-5v2Z"
        }
        Some("system") => "M3 3h8v8H3V3Zm10 0h8v5h-8V3ZM3 13h5v8H3v-8Zm7 3h11v5H10v-5Z",
        _ => {
            "M12 2a10 10 0 1 0 10 10A10.011 10.011 0 0 0 12 2Zm7.93 9h-3.01a15.58 15.58 0 0 0-1.18-5.03A8.01 8.01 0 0 1 19.93 11ZM12 4c1.1 0 2.72 2.08 3.03 7H8.97C9.28 6.08 10.9 4 12 4Zm-3.74 1.97A15.58 15.58 0 0 0 7.08 11H4.07a8.01 8.01 0 0 1 4.19-5.03ZM4.07 13h3.01a15.58 15.58 0 0 0 1.18 5.03A8.01 8.01 0 0 1 4.07 13ZM12 20c-1.1 0-2.72-2.08-3.03-7h6.06c-.31 4.92-1.93 7-3.03 7Zm3.74-1.97A15.58 15.58 0 0 0 16.92 13h3.01a8.01 8.01 0 0 1-4.19 5.03Z"
        }
    }
}
