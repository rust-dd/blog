use dioxus::prelude::*;

use crate::{
    app::Route,
    components::{icons, theme_toggle},
};

#[component]
pub fn Component() -> Element {
    rsx! {
        header { class: "sticky top-0 right-0 left-0 z-50 border-b border-dashed border-border bg-bg/80 backdrop-blur-md",
            div { class: "mx-auto max-w-4xl px-4 py-3 sm:px-6",
                div { class: "flex items-center justify-between",
                    Link {
                        to: Route::Home {},
                        class: "group flex items-center gap-1 text-sm text-fg transition-colors duration-200",
                        span { class: "text-accent", "~$" }
                        span { class: "ml-1 font-semibold", "rust-dd" }
                        span { class: "ml-0.5 inline-block h-4 w-2 animate-pulse bg-accent" }
                    }

                    div { class: "flex items-center gap-4 sm:gap-6",
                        Link {
                            to: Route::Projects {},
                            class: "nav-link text-sm text-muted transition-colors duration-200 hover:text-fg",
                            "projects"
                        }
                        Link {
                            to: Route::OpenSource {},
                            class: "nav-link text-sm text-muted transition-colors duration-200 hover:text-fg",
                            "open source"
                        }
                        span { class: "hidden h-4 w-px bg-border sm:block" }
                        div { class: "hidden sm:block",
                            icons::Component {}
                        }
                        theme_toggle::Component {}
                    }
                }
            }
        }
    }
}
