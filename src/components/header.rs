use dioxus::prelude::*;

use crate::{app::Route, components::icons};

#[component]
pub fn Component() -> Element {
    rsx! {
        header { class: "fixed top-0 right-0 left-0 z-50 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md",
            div { class: "container mx-auto max-w-5xl",
                div { class: "flex flex-row justify-between items-center text-white",
                    div { class: "flex flex-row gap-4",
                        Link {
                            to: Route::Home {},
                            class: "text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#67e8f9]",
                            "blog"
                        }
                        a {
                            href: "https://shrtn.ink/",
                            rel: "noopener noreferrer",
                            target: "_blank",
                            class: "text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#67e8f9]",
                            "shrtn.ink"
                        }
                    }
                    div { class: "hidden md:block",
                        icons::Component {}
                    }
                }
            }
        }
    }
}
