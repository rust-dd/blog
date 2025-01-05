use leptos::{
    html::{div, img, p},
    prelude::*,
};

#[component]
pub fn Component() -> impl IntoView {
    div()
        .class("flex absolute inset-0 flex-col gap-1 justify-center items-center m-auto")
        .child((
            img().src("/rust_color.webp").width(32).height(32).class("animate-spin"),
            p().class("text-sm italic text-muted-foreground").child("Loading..."),
        ))
}
