use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::{increment_views, select_post};
use crate::loader;

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
        #[cfg(not(debug_assertions))]
        if post.get().is_some() {
            increment_view.dispatch(post.get().as_ref().unwrap().id.id.to_string());
        }
    });

    view! {
        <Suspense fallback=|| {
            view! { <loader::Component /> }
        }>
            {move || {
                post.with(|post| {
                    let post = post.clone().unwrap_or_default();
                    view! {
                        <Show
                            when={
                                let post_header = post.header_image.clone();
                                move || post_header.is_some()
                            }
                            fallback=|| ()
                        >
                            <img
                                src=post.header_image.as_ref().unwrap().to_string()
                                alt=""
                                class="object-contain self-center mb-6 h-full rounded-lg w-fit aspect-auto"
                            />
                        </Show>
                        <Title text=post.title.to_string() />
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
                                class="my-6 mx-auto max-w-3xl prose prose-h3:text-white prose-h4:text-white prose-code:before:content-none prose-th:text-white prose-li:marker:text-white prose-code:after:content-none prose-pre:bg-transparent prose-pre:rounded-lg prose-pre:p-0 prose-code:text-[#ffbd2e] prose-strong:text-white prose-table:text-white prose-thead:text-white prose-li:text-white prose-ol:text-white prose-h1:text-white prose-h1:text-3xl prose-h2:text-white prose-h2:text-2xl prose-ul:text-white prose-p:text-white prose-a:text-[#ffbd2e]"
                                inner_html=post.body
                            />
                        </article>
                    }
                })
            }}
            <script
                src="https://giscus.app/client.js"
                data-repo="rust-dd/blog"
                data-repo-id="R_kgDOMRLPjw"
                data-category="General"
                data-category-id="DIC_kwDOMRLPj84CjCwK"
                data-mapping="title"
                data-strict="0"
                data-reactions-enabled="1"
                data-emit-metadata="0"
                data-input-position="bottom"
                data-theme="noborder_gray"
                data-lang="en"
                crossorigin="anonymous"
                async
            ></script>
        </Suspense>
    }
}
