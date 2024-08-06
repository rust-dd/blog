use leptos::*;
use leptos_router::use_navigate;

/// Renders the home page of your application.
#[component]
pub fn Component() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <For
            each=move || 0..10
            key=|i| i.to_string()
            children=move |i| {
                let value = navigate.clone();
                view! {
                    <div
                        on:click=move |_| { value(&format!("/post/{}", i), Default::default()) }
                        class="cursor-pointer hover:bg-gray-200 p-2"
                    >
                        {i}
                    </div>
                }
            }
        />
    }
}
