use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::{increment_views, select_post};

#[component]
pub fn Component() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap_or_default());
    let post = create_blocking_resource(
        || (),
        move |_| async move { select_post(slug()).await.unwrap() },
    );
    let increment_view = create_action(move |id: &String| {
        let id = id.clone();
        async move {
            let _ = increment_views(id.to_string()).await;
        }
    });
    create_effect(move |_| {
        //#[cfg(not(debug_assertions))]
        if post.get().is_some() {
            increment_view.dispatch(post.get().as_ref().unwrap().id.id.to_string());
        }
    });

    view! {
        <Suspense fallback=|| ()>
            {move || {
                post.with(|post| {
                    let post = post.clone().unwrap_or_default();
                    view! {
                        <Title text=post.title.to_string() />
                        <Meta name="description" content=post.summary.to_string() />
                        <Meta property="og:type" content="article" />
                        <Meta property="og:title" content=post.title.to_string() />
                        <Meta property="og:description" content=post.summary.to_string() />
                        {post
                            .tags
                            .into_iter()
                            .map(|tag| {
                                view! { <Meta name="keywords" content=tag.to_string() /> }
                            })
                            .collect::<Vec<_>>()}
                        <article>
                            <div class="flex flex-col gap-4 mx-auto max-w-3xl">
                                <p class="text-4xl font-semibold">{post.title.clone()}</p>
                                <div class="flex gap-3 justify-start items-center text-sm text-muted-foreground">
                                    <p
                                        on:click=move |e| {
                                            e.stop_propagation();
                                            if let Some(github) = &post.author.github {
                                                let _ = window().open_with_url_and_target(github, "_blank");
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
                                class="my-6 mx-auto max-w-3xl prose prose-h3:text-white prose-code:before:content-none prose-code:after:content-none prose-pre:bg-transparent prose-pre:rounded-lg prose-pre:p-0 prose-code:text-[#ffbd2e] prose-strong:text-white prose-h1:text-white prose-h1:text-3xl prose-h2:text-white prose-h2:text-2xl prose-ul:text-white prose-p:text-white prose-a:text-[#ffbd2e]"
                                inner_html=post.body
                            />
                        </article>
                    }
                })
            }}
        </Suspense>
    }
}
