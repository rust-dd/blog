use leptos::prelude::*;

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <div class="flex absolute inset-0 flex-col gap-1 justify-center items-center m-auto">
            <img src="/rust_color.webp" width=32 height=32 class="animate-spin" />
            <p class="text-sm italic text-muted-foreground">Loading...</p>
        </div>
    }
}
