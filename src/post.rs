use leptos::*;
use leptos_router::*;
use logging::log;

use crate::posts::select_post;

/// Renders the home page of your application.
#[component]
pub fn Component() -> impl IntoView {
    // Creates a reactive value to update the button
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let post = create_blocking_resource(|| (), move |_| async move { select_post(id()).await });

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                post.with(|post| {
                    let post = post.as_ref().unwrap();
                    let post = post.as_ref().unwrap();
                    view! {
                        <div>
                            <h1>{post.title.clone()}</h1>
                            <p>{post.author.name.clone()}</p>
                            <p>{post.read_time}min read</p>

                        </div>
                    }
                })
            }}
        // <article class="p-6 rounded-lg shadow-sm transition-transform duration-300 bg-card">
        // <div class="flex justify-between items-center mb-4">
        // <h2 class="text-2xl font-semibold">{&post.title}</h2>
        // <div class="text-sm text-muted-foreground">
        // {format!("{} min read", post.read_time)}
        // </div>
        // </div>
        // <div class="text-muted-foreground mb-4">
        // <span>{"by "}</span>
        // <span class="font-semibold ml-1">{&post.author.name}</span>
        // </div>
        // <div class="prose" inner_html=html_output></div>
        // </article>
        </Suspense>
    }
}
