use leptos::*;
use leptos_router::use_navigate;

use crate::api::{increment_views, select_posts, select_tags};

#[component]
pub fn Component() -> impl IntoView {
    let navigate = use_navigate();
    let selected_tags = create_rw_signal(Vec::<String>::new());
    let tags = create_blocking_resource(
        || (),
        move |_| async move { select_tags().await.unwrap_or_default() },
    );
    let posts = create_blocking_resource(
        move || selected_tags.get(),
        move |selected_tags| async move { select_posts(selected_tags).await.unwrap_or_default() },
    );
    let increment_view = create_action(move |id: &String| {
        let id = id.clone();
        async move {
            let _ = increment_views(id.to_string()).await;
        }
    });

    view! {
        <Suspense fallback=|| ()>
            {
                let navigate = navigate.clone();
                view! {
                    <div class="flex flex-row flex-wrap gap-1 px-4 text-xs">
                        <button
                            on:click=move |_| selected_tags.update(|prev| prev.clear())
                            class="py-1 px-2 text-white rounded-lg transition-all duration-500 cursor-pointer bg-primary"
                            class=("underline", move || selected_tags.get().is_empty())
                        >
                            {"All"}
                        </button>
                        <For
                            each=move || tags.get().unwrap_or_default()
                            key=|tag| tag.clone()
                            children=move |tag| {
                                view! {
                                    <button
                                        on:click={
                                            let tag = tag.clone();
                                            move |_| {
                                                selected_tags
                                                    .update(|prev| {
                                                        if prev.contains(&tag) {
                                                            *prev = prev
                                                                .clone()
                                                                .into_iter()
                                                                .filter(|v| v != &tag)
                                                                .collect::<Vec<_>>();
                                                        } else {
                                                            *prev = prev
                                                                .clone()
                                                                .into_iter()
                                                                .chain(std::iter::once(tag.clone()))
                                                                .collect();
                                                        }
                                                    });
                                            }
                                        }
                                        class="py-1 px-2 rounded-lg transition-all duration-500 cursor-pointer hover:text-black hover:bg-white"
                                        class=(
                                            "bg-white",
                                            {
                                                let tag = tag.clone();
                                                move || selected_tags.get().contains(&tag)
                                            },
                                        )
                                        class=(
                                            "text-black",
                                            {
                                                let tag = tag.clone();
                                                move || selected_tags.get().contains(&tag)
                                            },
                                        )
                                    >
                                        {tag}
                                    </button>
                                }
                            }
                        />
                    </div>
                    <For
                        each=move || posts.get().unwrap_or_default()
                        key=|post| post.id.id.to_string()
                        children=move |post| {
                            let navigate = navigate.clone();
                            view! {
                                <article
                                    on:click=move |_| {
                                        #[cfg(not(debug_assertions))]
                                        increment_view.dispatch(post.id.id.to_string());
                                        navigate(
                                            &format!("/post/{}", post.slug.as_ref().map_or("", |v| v)),
                                            Default::default(),
                                        );
                                    }
                                    class="p-6 rounded-lg shadow-sm transition-transform duration-300 cursor-pointer hover:shadow-lg hover:-translate-y-2 bg-card"
                                >
                                    <div class="flex gap-8 justify-between items-center mb-4">
                                        <p class="text-3xl font-semibold">
                                            {&post.title.to_string()}
                                        </p>

                                    </div>
                                    <p class="mb-2 text-muted-foreground">
                                        {&post.summary.to_string()}
                                    </p>
                                    <div class="flex gap-3 justify-end items-center text-sm text-muted-foreground">
                                        <p>{format!("{} min read", post.read_time)}</p>
                                        <p>{format!("{} views", post.total_views)}</p>
                                        <p>{post.created_at}</p>
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
                                    </div>
                                </article>
                            }
                        }
                    />
                }
            }
        </Suspense>
    }
}
