use crate::components::icons;
use leptos::{
    ev,
    html::{a, div, header},
    prelude::*,
};

pub fn component() -> impl IntoView {
    header()
        .class("fixed top-0 right-0 left-0 z-99 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md")
        .child(
            div().class("container mx-auto max-w-5xl").child(
                div()
                    .class("flex flex-row justify-between items-center text-white")
                    .child((
                        div().class("flex flex-row gap-4").child((
                            a().href("/")
                                .on(ev::click, move |_| {
                                    use web_sys::window;
                                    let document = window().unwrap().document().unwrap();
                                    if let Some(element) = document.get_element_by_id("giscus") {
                                        if let Some(parent) = element.parent_node() {
                                            parent.remove_child(&element).unwrap();
                                        }
                                    }
                                })
                                .class("text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]")
                                .child("blog"),
                            a().href("/references")
                                .class("text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]")
                                .child("references"),
                            a().href("/hireus")
                                .class("text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]")
                                .child("hire us!"),
                        )),
                        div().class("hidden md:block").child(icons::component),
                    )),
            ),
        )
}
