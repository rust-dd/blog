use leptos::*;
use leptos_router::use_navigate;

use crate::api::{select_posts, select_tags};

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

    view! {
        <Suspense fallback=|| ()>
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
        </Suspense>
        <Suspense fallback=|| ()>
            {
                let navigate = navigate.clone();
                view! {
                    <For
                        each=move || posts.get().unwrap_or_default()
                        key=|post| post.id.id.to_string()
                        children=move |post| {
                            let navigate = navigate.clone();
                            view! {
                                <article
                                    on:click=move |_| {
                                        navigate(
                                            &format!("/post/{}", post.slug.as_ref().map_or("", |v| v)),
                                            Default::default(),
                                        );
                                    }
                                    class="flex flex-col-reverse gap-10 p-6 rounded-lg shadow-sm transition-transform duration-300 cursor-pointer md:flex-row hover:shadow-lg hover:-translate-y-2 bg-card"
                                >
                                    <div>
                                        <div class="flex gap-8 justify-between items-center mb-4">
                                            <p class="text-3xl font-semibold">{post.title}</p>
                                        </div>
                                        <p class="mb-2 text-muted-foreground">{post.summary}</p>
                                        <div class="flex gap-3 justify-end items-center text-sm text-muted-foreground">
                                            <p>{format!("{} min read", post.read_time)}</p>
                                            <p>{format!("{} views", post.total_views)}</p>
                                            <p>{post.created_at}</p>
                                            <a
                                                href=post.author.github.unwrap_or_default()
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                }
                                                class="cursor-pointer hover:underline"
                                            >
                                                {"by "}
                                                <span class="ml-1 font-semibold">{post.author.name}</span>
                                            </a>
                                        </div>
                                    </div>
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
                                            class="object-contain w-full h-auto rounded-lg md:w-1/5 aspect-auto"
                                        />
                                    </Show>
                                </article>
                            }
                        }
                    />
                }
            }
        </Suspense>
    }
}
