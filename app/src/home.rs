use icondata as i;
use leptos::html::{button, div, p, span};
use leptos::{ev, prelude::*};
use leptos_icons::{Icon, IconProps};
use leptos_meta::{Title, TitleProps};
use leptos_router::components::{AProps, A};

use crate::{
    api::{select_posts, select_tags},
    components::loader,
};

pub fn component() -> impl IntoView {
    let selected_tags = RwSignal::new(Vec::<String>::new());
    let tags = Resource::new_blocking(|| (), move |_| async move { select_tags().await.unwrap_or_default() });
    let posts = Resource::new(
        move || selected_tags.get(),
        move |selected_tags| async move { select_posts(selected_tags).await },
    );

    div().child((
        Title(
            TitleProps::builder()
                .text("Rust-DD Blog – Tech Insights & Consulting")
                .build(),
        ),
        Suspense(
            SuspenseProps::builder().fallback(|| ()).children(TypedChildren::to_children(move || {
                div()
                    .class("gap-4 columns-1 sm:columns-2")
                    .child(For(ForProps::builder()
                        .each(move || posts.get().and_then(Result::ok).unwrap_or_default())
                        .key(|post| post.id.id.to_string())
                        .children(|post| {
                            div().class("flex flex-col p-3 text-left text-white rounded-lg transition-all duration-500 cursor-pointer break-inside-avoid bg-card hover:text-[#ffef5c]").child(
                                    A(AProps::builder()
                                        .href(format!("/post/{}", post.slug.as_ref().map_or("", |v| v)))
                                        .children(ToChildren::to_children(move || {
                                            div()
                                                .child(
                                                (div().class("flex flex-col gap-1 mb-4 font-medium").child((
                                                p().class("text-base line-clamp-2").child(post.title.clone()),
                                                p().class("italic text-xxs").child(post.summary.clone()),
                                            )),
                                            div().class("flex flex-row gap-3 justify-start items-center text-xxs").child(
                                                div().class("flex flex-row gap-3").child((
                                                    div().class("flex flex-row gap-1 items-center").child((
                                                        Icon(IconProps::builder()
                                                            .icon(Signal::from(i::FaClockSolid))
                                                            .width("1em")
                                                            .height("1em")
                                                            .build()),
                                                        p().child(format!("{} min read", post.read_time)),
                                                    )),
                                                    div().class("flex flex-row gap-1 items-center").child((
                                                        Icon(IconProps::builder()
                                                            .icon(Signal::from(i::FaEyeSolid))
                                                            .width("1em")
                                                            .height("1em")
                                                            .build()),
                                                        p().child(format!("{} views", post.total_views)),
                                                    )),
                                                    div().class("flex flex-row gap-1 items-center").child((
                                                        Icon(IconProps::builder()
                                                            .icon(Signal::from(i::FaCalendarSolid))
                                                            .width("1em")
                                                            .height("1em")
                                                            .build()),
                                                        p().child(post.created_at),
                                                    )),
                                                    div().class("flex flex-row gap-1 items-center").child((
                                                        Icon(IconProps::builder()
                                                            .icon(Signal::from(i::FaUserSolid))
                                                            .width("1em")
                                                            .height("1em")
                                                            .build()),
                                                        button().on(ev::click, move |e| {
                                                            e.prevent_default();
                                                            e.stop_propagation();
                                                            let _ = window().open_with_url_and_target(
                                                                post.author.github.as_ref().unwrap_or(&"".to_string()),
                                                                "_blank",
                                                            );
                                                        }).child(
                                                            span().class("text-xs font-semibold cursor-pointer hover:underline").child(post.author.name.clone()),
                                                        ),
                                                    )),
                                                )),
                                            )
                                        ))}))
                            .build())
                        )})
                    .build()))
                })
            ).build(),
        ),
        Suspense(SuspenseProps::builder().fallback(|| loader::component).children(TypedChildren::to_children(move || {
            div().class("flex flex-row flex-wrap gap-1 px-4 text-xs").child((
                button().on(ev::click, move |_| selected_tags.update(|prev| prev.clear()))
                    .class("py-1 px-2 text-white rounded-lg transition-all duration-500 cursor-pointer bg-primary")
                    .class(("underline", move || selected_tags.get().is_empty()))
                    .child("All"),
                For(ForProps::builder()
                    .each(move || tags.get().unwrap_or_default())
                    .key(|tag| tag.clone())
                    .children(move |(tag, count)| {
                        button().on(ev::click, {
                            let tag = tag.clone();
                            move |_| {
                                selected_tags.update(|prev| {
                                    if prev.contains(&tag) {
                                        *prev = prev.clone().into_iter().filter(|v| v != &tag).collect::<Vec<_>>();
                                    } else {
                                        *prev = prev.clone().into_iter().chain(std::iter::once(tag.clone())).collect();
                                    }
                                });
                            }
                        })
                        .class("py-1 px-2 rounded-lg transition-all duration-500 cursor-pointer hover:text-black hover:bg-white")
                        .class((
                            "bg-white",
                            {
                                let tag = tag.clone();
                                move || selected_tags.get().contains(&tag)
                            }),
                        )
                        .class((
                            "text-black",
                            {
                                let tag = tag.clone();
                                move || selected_tags.get().contains(&tag)
                            }),
                        )
                        .child(tag.clone() + " (" + &count.to_string() + ")")
                    })
                    .build())
            ))
        })).build())
    ))
}
