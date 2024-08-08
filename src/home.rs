use leptos::*;
use leptos_router::use_navigate;

use crate::posts::select_posts;

/// Renders the home page of your application.
#[component]
pub fn Component() -> impl IntoView {
    let navigate = use_navigate();
    let posts = create_blocking_resource(
        || (),
        move |_| async move { select_posts().await.unwrap_or_default() },
    );

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
                                    on:click=move |_| navigate(
                                        &format!("/post/{}", post.id.id),
                                        Default::default(),
                                    )
                                    class="p-6 rounded-lg shadow-sm transition-transform duration-300 cursor-pointer hover:shadow-lg hover:-translate-y-2 bg-card"
                                >
                                    <div class="flex justify-between items-center mb-4">
                                        <h2 class="text-xl font-semibold">
                                            {&post.title.to_string()}
                                        </h2>
                                        <div class="text-sm text-muted-foreground">
                                            {format!("{} min read", post.read_time)}
                                        </div>
                                    </div>
                                    <p class="mb-2 text-muted-foreground">
                                        {&post.summary.to_string()}
                                    </p>
                                    <div class="flex justify-between items-center text-sm text-muted-foreground">
                                        <span>
                                            {"by "}
                                            <span class="ml-1 font-semibold">
                                                {&post.author.name.to_string()}
                                            </span>
                                        </span>
                                        <span>{format!("{} views", post.total_views)}</span>
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
