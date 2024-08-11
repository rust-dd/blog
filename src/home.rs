use leptos::*;
use leptos_router::*;

use crate::posts::{increment_views, select_posts};

/// Renders the home page of your application.
#[component]
pub fn Component() -> impl IntoView {
    let navigate = use_navigate();
    let posts = create_blocking_resource(
        || (),
        move |_| async move { select_posts().await.unwrap_or_default() },
    );
    let increment_view = create_action(move |id: &String| {
        let id = id.clone();
        async move {
            let _ = increment_views(id.to_string()).await;
        }
    });

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
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
                                        increment_view.dispatch(post.id.id.to_string());
                                        navigate(
                                            &format!(
                                                "/post/{}-{}",
                                                post.slug.as_ref().map_or("", |v| v),
                                                post.id.id.to_string(),
                                            ),
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
