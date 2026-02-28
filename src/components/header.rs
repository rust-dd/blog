use dioxus::prelude::*;

use crate::{app::Route, components::icons};

#[component]
pub fn Component() -> Element {
    rsx! {
        header { class: "sticky top-0 right-0 left-0 z-50 border-b border-dashed border-slate-300 bg-white/90 backdrop-blur-md",
            div { class: "mx-auto max-w-4xl px-4 py-3 sm:px-6",
                div { class: "flex items-center justify-between",
                    Link {
                        to: Route::Home {},
                        class: "group flex items-center gap-1 text-sm text-slate-700 transition-colors duration-200 hover:text-slate-900",
                        span { class: "text-slate-400", "~$" }
                        span { class: "ml-1 font-semibold", "rust-dd" }
                        span { class: "ml-0.5 inline-block w-2 h-4 bg-slate-700 animate-pulse" }
                    }

                    div { class: "flex items-center gap-6",
                        Link {
                            to: Route::Home {},
                            class: "text-sm text-slate-500 transition-colors duration-200 hover:text-slate-800",
                            "posts"
                        }
                        span { class: "hidden sm:block h-4 w-px bg-slate-200" }
                        div { class: "hidden sm:block",
                            icons::Component {}
                        }
                    }
                }
            }
        }
    }
}
