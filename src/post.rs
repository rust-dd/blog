use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn Component() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <span class="font-extrabold">"Welcome to Leptos!"</span>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
