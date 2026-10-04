use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn Sidebar(#[props(into, default)] meta: String, children: Element) -> Element {
    let mut open = use_signal(|| false);
    let meta = if meta.is_empty() {
        "Rust engineering notes".to_string()
    } else {
        meta
    };

    rsx! {
        aside { class: "shell-side",
            div { class: "side-head",
                Link { to: Route::Home { q: String::new() }, class: "crate-mark",
                    span { class: "crate-logo", aria_hidden: "true", "dd" }
                    span { class: "crate-text",
                        span { class: "crate-name", "rust_dd" }
                        span { class: "crate-meta", "{meta}" }
                    }
                }
                button {
                    r#type: "button",
                    class: "side-toggle",
                    aria_expanded: "{open}",
                    aria_controls: "side-nav",
                    onclick: move |_| open.toggle(),
                    if open() { "Close" } else { "Menu" }
                }
            }
            nav {
                id: "side-nav",
                aria_label: "Site",
                class: if open() { "side-nav is-open" } else { "side-nav" },
                {children}
                SideGroup { title: "More",
                    li { Link { to: Route::Projects {}, class: "side-link side-code text-mod", "projects" } }
                    li { Link { to: Route::OpenSource {}, class: "side-link side-code text-mod", "open_source" } }
                    li { a { href: "/rss.xml", class: "side-link side-code text-mod", "rss" } }
                }
            }
        }
    }
}

#[component]
pub fn SideGroup(#[props(into)] title: String, children: Element) -> Element {
    rsx! {
        section { class: "side-group",
            h2 { class: "side-title", "{title}" }
            ul { class: "side-list", {children} }
        }
    }
}

/// Lives in the router layout, so the field keeps focus when typing on another page jumps to the home list.
#[component]
pub fn SearchBar() -> Element {
    let route = use_route::<Route>();
    let input_nav = navigator();
    let key_nav = navigator();
    let (value, on_home) = match &route {
        Route::Home { q } => (q.clone(), true),
        _ => (String::new(), false),
    };

    rsx! {
        div { class: "search", role: "search",
            label { r#for: "search", class: "sr-only", "Search posts, modules and authors" }
            input {
                id: "search",
                r#type: "search",
                class: "search-input",
                value: "{value}",
                placeholder: "Search posts, modules and authors",
                autocomplete: "off",
                spellcheck: "false",
                oninput: move |event| {
                    let target = Route::Home { q: event.value() };
                    if on_home {
                        input_nav.replace(target);
                    } else {
                        input_nav.push(target);
                    }
                },
                onkeydown: move |event| {
                    if on_home && event.key() == Key::Escape {
                        key_nav.replace(Route::Home { q: String::new() });
                    }
                },
            }
            kbd { class: "search-key", aria_hidden: "true", "S" }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "shell-footer",
            p { "Rust-DD is Daniel Boros and Daniel Zelei." }
            nav { aria_label: "Elsewhere", class: "footer-links",
                a { href: "/rss.xml", "RSS" }
                a { href: "https://github.com/rust-dd", rel: "noopener noreferrer", target: "_blank", "GitHub" }
                a { href: "https://www.linkedin.com/company/rust-dd", rel: "noopener noreferrer", target: "_blank", "LinkedIn" }
                a { href: "https://x.com/rust_dd", rel: "noopener noreferrer", target: "_blank", "X" }
                a { href: "mailto:info@rust-dd.com", "info@rust-dd.com" }
            }
        }
    }
}

#[component]
pub fn SectionHeading(#[props(into)] id: String, #[props(into)] title: String) -> Element {
    rsx! {
        h2 { id: "{id}", class: "doc-h2",
            a { class: "doc-anchor", href: "#{id}", aria_label: "Link to {title}", "§" }
            "{title}"
        }
    }
}
