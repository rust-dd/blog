use leptos::*;
use leptos_router::*;
use pulldown_cmark::{html, Parser};

use crate::posts::select_post;

#[component]
pub fn Component() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params.with(|params| {
            params
                .get("slug")
                .cloned()
                .unwrap_or_default()
                .split('-')
                .last()
                .unwrap_or_default()
                .to_string()
        })
    };
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
                        <article class="rounded-lg shadow-sm transition-transform duration-300 bg-card">
                            <div class="flex flex-col gap-4">
                                <p class="text-4xl font-semibold">{post.title.clone()}</p>
                                <div class="flex gap-3 justify-start items-center text-sm text-muted-foreground">
                                    <p
                                        on:click=move |e| {
                                            e.stop_propagation();
                                            if let Some(github) = &post.author.github {
                                                let _ = window()
                                                    .open_with_url_and_target(&github, "_blank");
                                            }
                                        }
                                        class="cursor-pointer hover:underline"
                                    >
                                        {"by "}
                                        <span class="ml-1 font-semibold">
                                            {&post.author.name.to_string()}
                                        </span>
                                    </p>
                                    <p>{post.created_at}</p>
                                    <p>{format!("{} min read", post.read_time)}</p>
                                    <p>{format!("{} views", post.total_views)}</p>
                                </div>
                            </div>
                            <div
                                class="my-6 prose prose-h3:text-white prose-code:before:content-none prose-code:after:content-none prose-code:text-[#ffbd2e] prose-strong:text-white prose-h1:text-white prose-h1:text-3xl prose-h2:text-white prose-h2:text-2xl prose-ul:text-white prose-p:text-white prose-a:text-[#ffbd2e]"
                                inner_html=html_output
                            />
                        </article>
                    }
                })}
        </Suspense>
    }
}
