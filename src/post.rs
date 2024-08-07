use leptos::*;
use leptos_router::*;
use pulldown_cmark::{html, Parser};

use crate::posts::select_post;

#[component]
pub fn Component() -> impl IntoView {
    // Creates a reactive value to update the button
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let post = create_blocking_resource(
        || (),
        move |_| async move { select_post(id()).await.unwrap() },
    );

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {post
                .with(|post| {
                    let post = post.clone().unwrap_or_default();
                    let markdown_input = post.body.to_string();
                    let parser = Parser::new(&markdown_input);
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);
                    view! {
                        <article class="p-6 rounded-lg shadow-sm transition-transform duration-300 bg-card">
                            <div class="flex justify-between items-center mb-4">
                                <h2 class="text-2xl font-semibold">{post.title.clone()}</h2>
                                <div class="text-sm text-muted-foreground">
                                    {format!("{} min read", post.read_time)}
                                </div>
                            </div>
                            <div class="text-muted-foreground mb-4">
                                <span>{"by "}</span>
                                <span class="font-semibold ml-1">{post.author.name.clone()}</span>
                            </div>
                            <div class="prose prose-xl text-white" inner_html=html_output />
                        </article>
                    }
                })}
        </Suspense>
    }
}
