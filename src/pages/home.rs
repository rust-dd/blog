use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::components::loader;
use crate::ssr::api::{select_posts, select_tags};

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
        <Title text="Rust-DD Blog – Tech Insights & Consulting" />
        <Suspense fallback=|| ()>
            <div class="gap-4 mb-20">
                <For
                    each=move || posts.get().and_then(Result::ok).unwrap_or_default()
                    key=|post| post.id.id.to_string()
                    let:post
                >
                    <div class="flex flex-col p-3 text-left text-white rounded-lg transition-all duration-500 cursor-pointer break-inside-avoid bg-card hover:text-[#ffef5c]">
                        <A href=format!("/post/{}", post.slug.as_ref().map_or("", |v| v))>
                            <div class="flex flex-col gap-1 w-full font-thin">
                                <p class="text-xs text-[#969696]">{post.created_at}</p>
                                <p class="text-xs underline line-clamp-2">{post.title}</p>
                            </div>
                        </A>
                    </div>
                </For>
            </div>
        </Suspense>
    }
}
