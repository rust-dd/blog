use crate::{
    components::{error_template, header, icons},
    pages::{hireus, home, post, references},
};
use chrono::{Datelike, Utc};
use leptos::{
    html::{a, body, div, footer, head, html, main, meta, p},
    prelude::*,
};
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, FlatRoutesProps, Route, RouteChildren, RouteProps, Router, RouterProps},
    ParamSegment, SsrMode, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let html_ = html().lang("en").child((
        head().child((
            meta().charset("utf-8"),
            meta()
                .name("viewport")
                .content("width=device-width, initial-scale=1"))).child(
            AutoReload(AutoReloadProps::builder().options(options.clone()).build())).child(
            HydrationScripts(HydrationScriptsProps::builder().options(options).build())).child(
            MetaTags()).child((
            Stylesheet(
                StylesheetProps::builder()
                    .id("leptos")
                    .href("/pkg/blog.css")
                    .build(),
            ),
            Stylesheet(
                StylesheetProps::builder()
                    .id("katex")
                    .href("/katex.min.css")
                    .build(),
            ))).child(
            Title(
                TitleProps::builder()
                    .text("Rust-DD Blog – Tech Insights & Consulting")
                    .build(),
            )).child((
            Meta(
                MetaProps::builder()
                    .name("hostname")
                    .content("rust-dd.com")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("expected-hostname")
                    .content("rust-dd.com")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("description")
                    .content(
                        "Explore open-source Rust projects, learn innovative techniques, and connect with a passionate community. Get expert Rust development and consulting services.",
                    )
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("keywords")
                    .content("rust-dd, rust, ai, mathematics, embedded, web, systems, programming")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("robots")
                    .content("index, follow")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("googlebot")
                    .content("index, follow")
                    .build(),
            ))).child((
            // Facebook
            Meta(
                MetaProps::builder()
                    .property("og:type")
                    .content("website")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .property("og:title")
                    .content("Rust-DD Blog – Tech Insights & Consulting")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .property("og:site_name")
                    .content("Rust-DD Blog – Tech Insights & Consulting")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .property("og:description")
                    .content(
                        "Explore open-source Rust projects, learn innovative techniques, and connect with a passionate community. Get expert Rust development and consulting services.",
                    )
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .property("og:url")
                    .content("https://rust-dd.com/")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .property("og:image")
                    .content("https://static.rust-dd.com/rust-dd_custom_bg.png")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .property("og:image:type")
                    .content("image/png")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .property("og:image:width")
                    .content("1200")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .property("og:image:height")
                    .content("627")
                    .build(),
            ))).child((
            // Twitter
            Meta(
                MetaProps::builder()
                    .name("twitter:card")
                    .content("summary_large_image")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("twitter:title")
                    .content("Rust-DD Blog – Tech Insights & Consulting")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("twitter:description")
                    .content(
                        "Explore open-source Rust projects, learn innovative techniques, and connect with a passionate community. Get expert Rust development and consulting services.",
                    )
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("twitter:site")
                    .content("@rust_dd")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("twitter:url")
                    .content("https://rust-dd.com/")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("twitter:image")
                    .content("https://static.rust-dd.com/rust-dd_custom_bg.png")
                    .build(),
            ),
            Meta(
                MetaProps::builder()
                    .name("twitter:image:alt")
                    .content("Rust-DD logo")
                    .build(),
            ))).child((
            Link(
                LinkProps::builder()
                    .rel("preconnect")
                    .href("https://fonts.googleapis.com")
                    .build(),
            ),
            Link(
                LinkProps::builder()
                    .rel("preconnect")
                    .href("https://fonts.gstatic.com")
                    .build(),
        ))),
        body().class("bg-[#1e1e1e]").child(self::component),
    ));

    view! {
        <!DOCTYPE html>
        {html_}
    }
}

pub fn component() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    Router(
        RouterProps::builder()
            .children(TypedChildren::to_children(move || {
                div().class("overflow-auto text-white font-poppins").child((
          header::component,
          main()
            .class("container flex flex-col gap-8 px-4 pt-10 pb-14 mx-auto mt-16 max-w-4xl md:px-0")
            .child(FlatRoutes(
              FlatRoutesProps::builder()
                .fallback(|| {
                  let mut outside_errors = Errors::default();
                  outside_errors.insert_with_default_key(error_template::AppError::NotFound);
                  error_template::Component(
                    error_template::ComponentProps::builder()
                      .outside_errors(outside_errors)
                      .build(),
                  )
                })
                .children(RouteChildren::to_children(move || {
                  (
                    Route(
                      RouteProps::builder()
                        .path(StaticSegment(""))
                        .view(home::Component)
                        .ssr(SsrMode::InOrder)
                        .build(),
                    ),
                    Route(
                      RouteProps::builder()
                        .path(StaticSegment("references"))
                        .view(references::component)
                        .build(),
                    ),
                    Route(
                      RouteProps::builder()
                        .path(StaticSegment("hireus"))
                        .view(hireus::component)
                        .build(),
                    ),
                    Route(
                      RouteProps::builder()
                        .path((StaticSegment("post"), ParamSegment("slug")))
                        .view(post::component)
                        .ssr(SsrMode::Async)
                        .build(),
                    ),
                  )
                }))
                .build(),
            )),
          footer()
            .class("fixed right-0 bottom-0 left-0 z-10 py-2 text-center md:py-4 bg-[#1e1e1e]/80 backdrop-blur-md")
            .child(
              div().class("flex flex-col gap-1 justify-center items-center").child((
                p().class("text-gray-400").child((
                  "Powered by",
                  a()
                    .href("https://github.com/rust-dd")
                    .class("hover:underline text-[#ffef5c]")
                    .child(" rust-dd"),
                  format!(" © {}", Utc::now().year()),
                )),
                div().class("block md:hidden").child(icons::component),
              )),
            ),
        ))
            }))
            .build(),
    )
}
