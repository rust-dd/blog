use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::loader;
use crate::server::{select_posts, select_tags};

#[component]
pub fn Component() -> impl IntoView {
    let selected_tags = RwSignal::new(Vec::<String>::new());
    let tags = Resource::new_blocking(
        || (),
        move |_| async move { select_tags().await.unwrap_or_default() },
    );
    let posts = Resource::new(
        move || selected_tags.get(),
        move |selected_tags| async move { select_posts(selected_tags).await },
    );

    view! {
        <Title text="Tech Diaries - The Official Rust-DD Developer Blog" />
        <Suspense fallback=|| ()>
            <div class="gap-4 columns-1 sm:columns-2">
                <For
                    each=move || posts.get().and_then(Result::ok).unwrap_or_default()
                    key=|post| post.id.id.to_string()
                    let:post
                >
                    <div class="flex flex-col p-3 text-left text-white rounded-lg transition-all duration-500 cursor-pointer break-inside-avoid bg-card hover:text-[#ffef5c]">
                        <A href=format!("/post/{}", post.slug.as_ref().map_or("", |v| v))>
                            <div class="flex flex-col gap-1 mb-4 font-medium">
                                <p class="text-base line-clamp-2">{post.title}</p>
                                <p class="italic text-xxs">{post.summary}</p>
                            </div>
                            <div class="flex flex-row gap-3 justify-start items-center text-xxs">
                                <div class="flex flex-row gap-3">
                                    <div class="flex flex-row gap-1 items-center">
                                        <svg
                                            class="size-4"
                                            width="1em"
                                            height="1em"
                                            viewBox="0 0 512 512"
                                            fill="currentColor"
                                            role="graphics-symbol"
                                            data-hk="0-1-0-15"
                                        >
                                            <circle cx="256" cy="272" r="16"></circle>
                                            <path d="M280,81.5V72a24,24,0,0,0-48,0v9.5a191,191,0,0,0-84.43,32.13L137,103A24,24,0,0,0,103,137l8.6,8.6A191.17,191.17,0,0,0,64,272c0,105.87,86.13,192,192,192s192-86.13,192-192C448,174.26,374.58,93.34,280,81.5ZM256,320a48,48,0,0,1-16-93.25V152a16,16,0,0,1,32,0v74.75A48,48,0,0,1,256,320Z"></path>
                                        </svg>
                                        <p>{format!("{} min read", post.read_time)}</p>
                                    </div>
                                    <div class="flex flex-row gap-1 items-center">
                                        <svg
                                            class="size-4"
                                            width="1em"
                                            height="1em"
                                            viewBox="0 0 512 512"
                                            fill="currentColor"
                                            role="graphics-symbol"
                                            data-hk="0-1-0-20"
                                        >
                                            <circle cx="256" cy="256" r="64"></circle>
                                            <path d="M490.84,238.6c-26.46-40.92-60.79-75.68-99.27-100.53C349,110.55,302,96,255.66,96c-42.52,0-84.33,12.15-124.27,36.11C90.66,156.54,53.76,192.23,21.71,238.18a31.92,31.92,0,0,0-.64,35.54c26.41,41.33,60.4,76.14,98.28,100.65C162,402,207.9,416,255.66,416c46.71,0,93.81-14.43,136.2-41.72,38.46-24.77,72.72-59.66,99.08-100.92A32.2,32.2,0,0,0,490.84,238.6ZM256,352a96,96,0,1,1,96-96A96.11,96.11,0,0,1,256,352Z"></path>
                                        </svg>
                                        <p>{format!("{} views", post.total_views)}</p>
                                    </div>
                                    <div class="flex flex-row gap-1 items-center">
                                        <svg
                                            class="size-4"
                                            width="1em"
                                            height="1em"
                                            viewBox="0 0 512 512"
                                            fill="currentColor"
                                            role="graphics-symbol"
                                            data-hk="0-1-0-25"
                                        >
                                            <path d="M480,128a64,64,0,0,0-64-64H400V48.45c0-8.61-6.62-16-15.23-16.43A16,16,0,0,0,368,48V64H144V48.45c0-8.61-6.62-16-15.23-16.43A16,16,0,0,0,112,48V64H96a64,64,0,0,0-64,64v12a4,4,0,0,0,4,4H476a4,4,0,0,0,4-4Z"></path>
                                            <path d="M32,416a64,64,0,0,0,64,64H416a64,64,0,0,0,64-64V179a3,3,0,0,0-3-3H35a3,3,0,0,0-3,3ZM376,208a24,24,0,1,1-24,24A24,24,0,0,1,376,208Zm0,80a24,24,0,1,1-24,24A24,24,0,0,1,376,288Zm-80-80a24,24,0,1,1-24,24A24,24,0,0,1,296,208Zm0,80a24,24,0,1,1-24,24A24,24,0,0,1,296,288Zm0,80a24,24,0,1,1-24,24A24,24,0,0,1,296,368Zm-80-80a24,24,0,1,1-24,24A24,24,0,0,1,216,288Zm0,80a24,24,0,1,1-24,24A24,24,0,0,1,216,368Zm-80-80a24,24,0,1,1-24,24A24,24,0,0,1,136,288Zm0,80a24,24,0,1,1-24,24A24,24,0,0,1,136,368Z"></path>
                                        </svg>
                                        <p>{post.created_at}</p>
                                    </div>
                                </div>
                                <div class="flex flex-row gap-1 items-center">
                                    <svg
                                        class="size-4"
                                        width="1em"
                                        height="1em"
                                        viewBox="0 0 512 512"
                                        fill="currentColor"
                                        role="graphics-symbol"
                                        data-hk="0-1-0-30"
                                    >
                                        <path d="M332.64,64.58C313.18,43.57,286,32,256,32c-30.16,0-57.43,11.5-76.8,32.38-19.58,21.11-29.12,49.8-26.88,80.78C156.76,206.28,203.27,256,256,256s99.16-49.71,103.67-110.82C361.94,114.48,352.34,85.85,332.64,64.58Z"></path>
                                        <path d="M432,480H80A31,31,0,0,1,55.8,468.87c-6.5-7.77-9.12-18.38-7.18-29.11C57.06,392.94,83.4,353.61,124.8,326c36.78-24.51,83.37-38,131.2-38s94.42,13.5,131.2,38c41.4,27.6,67.74,66.93,76.18,113.75,1.94,10.73-.68,21.34-7.18,29.11A31,31,0,0,1,432,480Z"></path>
                                    </svg>
                                    <button on:click=move |e| {
                                        e.stop_propagation();
                                        let _ = window()
                                            .open_with_url_and_target(
                                                post.author.github.as_ref().unwrap_or(&"".to_string()),
                                                "_blank",
                                            );
                                    }>
                                        <span class="font-semibold cursor-pointer hover:underline">
                                            {post.author.name}
                                        </span>
                                    </button>
                                </div>
                            </div>
                        </A>
                    </div>
                </For>
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
                <For each=move || tags.get().unwrap_or_default() key=|tag| tag.clone() let:data>
                    <button
                        on:click={
                            let tag = data.0.clone();
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
                                let tag = data.0.clone();
                                move || selected_tags.get().contains(&tag)
                            },
                        )
                        class=(
                            "text-black",
                            {
                                let tag = data.0.clone();
                                move || selected_tags.get().contains(&tag)
                            },
                        )
                    >
                        {data.0.clone() + " (" + &data.1.to_string() + ")"}
                    </button>
                </For>
            </div>
        </Suspense>
    }
}
