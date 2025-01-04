use crate::components::icons;
use leptos::prelude::*;

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <header class="mt-14 z-10">
            <div class="container mx-auto max-w-4xl">
                <div class="flex flex-row justify-between items-center">
                    <div class="flex flex-row text-xs items-center py-2 px-4 text-[#969696]">
                        <p>{"rust-dd://"}</p>
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
                            class="transition-all duration-500 hover:underline hover:text-[#ffef5c]"
                        >
                            {"blog"}
                        </a>
                        <p>{"."}</p>
                        <a
                            href="/references"
                            class="transition-all duration-500 hover:underline hover:text-[#ffef5c]"
                        >
                            {"references"}
                        </a>
                        <p>{"."}</p>
                        <a
                            href="/hireus"
                            class="transition-all duration-500 hover:underline hover:text-[#ffef5c]"
                        >
                            {"hire.us"}
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
