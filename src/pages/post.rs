use leptos::html::{article, div, img, p, span};
use leptos::svg::script;
use leptos::{children, ev, prelude::*};
use leptos_meta::*;
use leptos_router::hooks::use_params_map;

use crate::components::loader;
use crate::ssr::api::{increment_views, select_post};
use crate::ssr::types::Post;

pub fn component() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").unwrap_or_default());
    let (show, set_show) = signal(false);
    let post = Resource::new_blocking(
        || (),
        move |_| async move {
            let post = select_post(slug()).await.unwrap();
            set_show(true);
            post
        },
    );
    let increment_view = Action::new(move |id: &String| {
        let id = id.clone();
        async move {
            let _ = increment_views(id.to_string()).await;
        }
    });
    Effect::new(move |_| {
        #[cfg(not(debug_assertions))]
        if post.get().is_some() {
            increment_view.dispatch(post.get().as_ref().unwrap().id.id.to_string());
        }
    });

    let children = move |post: Post| {
        div().child(
        (
            Show(ShowProps::builder()
                   .when({
                       let header_image = post.header_image.clone();
                       move || {
                           header_image.is_some()
                       }
                   })
                   .fallback(|| ())
                   .children(ToChildren::to_children(
                       move ||
                         img()
                           .alt("")
                           .class("object-contain self-center mb-6 h-full rounded-lg w-fit aspect-auto")
                           .src(post.header_image.clone().unwrap()))
                   ).build(),
            ),
            Title(TitleProps::builder().text(post.title.to_string()).build()),
            article().child((
                div().class("flex flex-col gap-4 mx-auto max-w-3xl").child((
                    p().class("text-4xl font-semibold").child(post.title.clone()),
                    div().class("flex gap-3 justify-start items-center text-sm text-muted-foreground").child((
                        p().on(ev::click, move |e| {
                            e.stop_propagation();
                            if let Some(github) = &post.author.github {
                                let _ = window().open_with_url_and_target(github, "_blank");
                            }
                        }).class("cursor-pointer hover:underline").child(("by ", span().class("ml-1 font-semibold").child(post.author.name.to_string()))),
                        p().child(post.created_at),
                        p().child(format!("{} min read", post.read_time)),
                        p().child(format!("{} views", post.total_views)),
                    )),
                )),
                div().class("my-6 mx-auto max-w-3xl prose prose-h3:text-white prose-h4:text-white prose-code:before:content-none prose-th:text-white prose-li:marker:text-white prose-code:after:content-none prose-pre:bg-transparent prose-pre:rounded-lg prose-pre:p-0 prose-code:text-[#ffef5c] prose-strong:text-white prose-table:text-white prose-thead:text-white prose-li:text-white prose-ol:text-white prose-h1:text-white prose-h1:text-3xl prose-h2:text-white prose-h2:text-2xl prose-ul:text-white prose-p:text-white prose-a:text-[#ffef5c]")
                  .inner_html(post.body.to_string())
            )),
            // div().id("giscus").child(Show(ShowProps::builder().when(show).children(ToChildren::to_children(move ||
            //   script()
            //     .prop("src", "https://giscus.app/client.js")
            //     .prop("data-repo", "rust-dd/blog")
            //     .prop("data-repo-id", "R_kgDOMRLPjw")
            //     .prop("data-category", "General")
            //     .prop("data-category-id", "DIC_kwDOMRLPj84CjCwK")
            //     .prop("data-mapping", "title")
            //     .prop("data-strict", "0")
            //     .prop("data-reactions-enabled", "1")
            //     .prop("data-emit-metadata", "0")
            //     .prop("data-input-position", "bottom")
            //     .prop("data-theme", "noborder_gray")
            //     .prop("data-lang", "en")
            //     .prop("crossorigin", "anonymous")
            //     .prop("async", "")
            // )).build()))
        ))
    };

    view! {
        <Suspense fallback=|| {
            view! { <loader::Component /> }
        }>
            {move || {
                post.with(|post| {
                    let post = post.clone().unwrap_or_default();
                    view! { {children(post)} }
                })
            }}
        </Suspense>
    }
}
