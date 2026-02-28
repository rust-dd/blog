use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn Component() -> Element {
    rsx! {
        header { class: "sticky top-0 right-0 left-0 z-50 px-4 py-2 border-b backdrop-blur-md bg-white/85 border-slate-200/80 sm:px-5 md:px-6",
            div { class: "container mx-auto max-w-6xl",
                Link {
                    to: Route::Home {},
                    class: "inline-flex text-sm font-semibold tracking-tight text-slate-900 transition-all duration-300 sm:text-2xl hover:text-slate-600",
                    "blog"
                }
            }
        }
    }
}
