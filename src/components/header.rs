use crate::components::icons;
use leptos::prelude::*;

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <header class="fixed top-0 right-0 left-0 z-10 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md">
            <div class="container mx-auto max-w-5xl">
                <div class="flex flex-row justify-between items-center text-white">
                    <div class="flex flex-row gap-4">
                        <a
                            href="/"
                            on:click=move |_| {
                                use web_sys::window;
                                let document = window().unwrap().document().unwrap();
                                if let Some(element) = document.get_element_by_id("giscus") {
                                    if let Some(parent) = element.parent_node() {
                                        parent.remove_child(&element).unwrap();
                                    }
                                }
                            }
                            class="text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]"
                        >
                            blog
                        </a>
                        <a
                            href="/references"
                            class="text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]"
                        >
                            references
                        </a>
                        <a
                            href="/hireus"
                            class="text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]"
                        >
                            hire us!
                        </a>
                    </div>
                    <div class="hidden md:block">
                        <icons::Component />
                    </div>
                </div>
            </div>
        </header>
    }
}
