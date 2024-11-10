use icondata as i;
use leptos::*;
use leptos_icons::Icon;
use leptos_meta::Title;

use crate::api::{select_posts, select_tags};
use crate::loader;

#[component]
pub fn Component() -> impl IntoView {
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
        <Title text="Tech Diaries - The Official Rust-DD Developer Blog" />
        <Suspense fallback=|| ()>
            <div class="gap-4 columns-1 sm:columns-2">
                <For
                    each=move || posts.get().unwrap_or_default()
                    key=|post| post.id.id.to_string()
                    children=move |post| {
                        view! {
                            <div class="flex flex-col p-3 text-left text-white rounded-lg transition-all duration-500 cursor-pointer break-inside-avoid bg-card hover:text-[#ffef5c]">
                                <a href=format!("/post/{}", post.slug.as_ref().map_or("", |v| v))>
                                    <div class="flex flex-col gap-1 mb-4">
                                        <p class="text-base font-medium line-clamp-2">
                                            {post.title}
                                        </p>
                                        <p class="italic text-xxs">{post.summary}</p>
                                    </div>
                                    <div class="flex flex-row gap-3 justify-start items-center text-xxs">
                                        <div class="flex flex-row gap-3">
                                            <div class="flex flex-row gap-1 items-center">
                                                <Icon icon=i::IoStopwatch class="size-4" />
                                                <p>{format!("{} min read", post.read_time)}</p>
                                            </div>
                                            <div class="flex flex-row gap-1 items-center">
                                                <Icon icon=i::IoEye class="size-4" />
                                                <p>{format!("{} views", post.total_views)}</p>
                                            </div>
                                            <div class="flex flex-row gap-1 items-center">
                                                <Icon icon=i::IoCalendar class="size-4" />
                                                <p>{post.created_at}</p>
                                            </div>
                                        </div>
                                        <div class="flex flex-row gap-1 items-center">
                                            <Icon icon=i::IoPerson class="size-4" />
                                            <a
                                                href=post.author.github.unwrap_or_default()
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                }
                                                class="cursor-pointer hover:underline"
                                            >
                                                <span class="font-semibold">{post.author.name}</span>
                                            </a>
                                        </div>
                                    </div>
                                </a>
                            </div>
                        }
                    }
                />
            </div>
        </Suspense>
        <Suspense fallback=|| view! { <loader::Component /> }>
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
                    children=move |(tag, count)| {
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
                                {tag + " (" + &count.to_string() + ")"}
                            </button>
                        }
                    }
                />
            </div>
        </Suspense>
    }
}
