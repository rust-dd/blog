use dioxus::prelude::*;

#[component]
pub fn Component() -> Element {
    rsx! {
        div { class: "flex absolute inset-0 flex-col gap-1 justify-center items-center m-auto",
            img { src: "/rust_color.webp", width: 32, height: 32, class: "animate-spin" }
            p { class: "text-sm italic text-muted-foreground", "Loading..." }
        }
    }
}

#[component]
pub fn Inline(message: Option<String>) -> Element {
    let message = message.unwrap_or_else(|| "Loading...".to_string());

    rsx! {
        div { class: "flex min-h-[55vh] flex-col gap-2 justify-center items-center",
            img { src: "/rust_color.webp", width: 28, height: 28, class: "animate-spin" }
            p { class: "text-sm italic text-slate-500", "{message}" }
        }
    }
}
