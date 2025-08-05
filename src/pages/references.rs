use crate::ssr::api::select_references;
use leptos::{
    html::{a, br, div, h1, h3, h4, p, section, span},
    prelude::*,
};
use leptos_icons::{Icon, IconProps};
use icondata as i;

pub fn component() -> impl IntoView {
    let references = LocalResource::new(move || select_references());
    let references = move || references.get().and_then(Result::ok).unwrap_or_default();

    fn which_icon(icon: &str) -> icondata::Icon {
        match icon {
            "database" => i::LuDatabase,
            "iot" => i::LuZap,
            "ai" => i::LuBrain,
            "math" => i::LuCpu,
            "system" => i::LuLayers,
            _ => i::LuGlobe,
        }
    }

    div().class("container py-12 px-4 mx-auto").child((
        div().class("absolute inset-0 overflow-hidden").child((
            div().class("absolute top-20 right-20 w-72 h-72 bg-[#ffef5c]/3 rounded-full blur-3xl"),
            div().class("absolute bottom-20 left-20 w-72 h-72 bg-[#ffef5c]/3 rounded-full blur-3xl"),
            div().class("absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-[#ffef5c]/2 rounded-full blur-3xl")
        )),
        section().id("about").class("text-center mb-16").child((
            div().class("inline-flex items-center gap-3 mb-8").child((
                div().class("w-8 h-px bg-[#ffef5c]"),
                span().class("text-[#ffef5c] font-mono text-sm tracking-widest").child("RUST PORTFOLIO"),
                div().class("w-8 h-px bg-[#ffef5c]")
            )),
            h1().class("text-7xl lg:text-9xl font-black text-white mb-6 leading-none").child((
                span().child("PROJECT"),
                br(),
                span().class("text-[#ffef5c]").child("SHOWCASE"),
            )),
            p().class("text-xl text-gray-300 max-w-2xl mx-auto leading-relaxed").child("High-performance Rust applications spanning big data processing, AI/ML systems, and real-time streaming solutions."),
            div().class("flex items-center justify-center gap-8 mt-8").child((
                div().class("text-center").child((
                    div().class("text-2xl font-bold text-[#ffef5c]").child(move || references().len()),
                    div().class("text-sm text-gray-400").child("Projects")
                )),
                div().class("w-px h-8 bg-gray-600"),
                div().class("text-center").child((
                    div().class("text-2xl font-bold text-[#ffef5c]").child("100%"),
                    div().class("text-sm text-gray-400").child("Rust")
                )),
                div().class("w-px h-8 bg-gray-600"),
                div().class("text-center").child((
                    div().class("text-2xl font-bold text-[#ffef5c]").child("2025"),
                    div().class("text-sm text-gray-400").child("Latest")
                ))
            ))
        )),
        section().id("projects").class("grid lg:grid-cols-2 gap-8 max-w-7xl mx-auto").child(
                Suspense(
                    SuspenseProps::builder()
                        .fallback(|| ())
                        .children(TypedChildren::to_children(move || {
                            For(
                                ForProps::builder()
                                    .each(move || references().into_iter().enumerate())
                                    .key(|(index, r)| format!("{}-{}", index, r.id.id))
                                    .children(|(index, r)| {
                                        div().class("group relative bg-gradient-to-br from-white/8 to-white/4 rounded-2xl border border-white/10 hover:border-[#ffef5c]/30 transition-all duration-500 overflow-hidden").child((
                                            div().class("absolute top-4 right-4 z-10").child(
                                                span().class("text-6xl font-black text-[#ffef5c]/10 group-hover:text-[#ffef5c]/20 transition-colors duration-500").child(format!("{:02}", index + 1))
                                            ),
                                            div().class("p-8 border-b border-white/10 relative z-20").child((
                                                div().class("flex items-start justify-between mb-4").child((
                                                    div().class("flex items-center gap-4").child((
                                                       div().class("w-14 h-14 bg-[#ffef5c]/10 rounded-xl flex items-center justify-center border border-[#ffef5c]/20 group-hover:bg-[#ffef5c]/20 transition-colors duration-300")
                                                           .child(
                                                               div().class("text-[#ffef5c]").child(
                                                               Icon(
                                                                   IconProps::builder()
                                                                       .icon(Signal::from(which_icon(r.icon.clone().unwrap_or("".to_string()).as_str())))
                                                                       .width("1.5em")
                                                                       .height("1.5em")
                                                                       .build(),
                                                               ))
                                                           ),
                                                       div().child((
                                                           div().class("text-sm text-[#ffef5c] font-mono mb-1").child(r.year),
                                                           div().class("text-sm text-gray-400").child(r.category)
                                                       ))
                                                    )),
                                                    Show(
                                                        ShowProps::builder()
                                                            .when(move || !r.url.is_empty())
                                                            .fallback(|| ())
                                                            .children(ToChildren::to_children(
                                                                || a().class("p-2 rounded-lg bg-white/5 hover:bg-[#ffef5c]/10 transition-colors duration-300 group/link")
                                                                    .child(
                                                                        div().class("w-5 h-5 text-gray-400 group-hover/link:text-[#ffef5c] transition-colors duration-300").child(
                                                                        Icon(
                                                                            IconProps::builder()
                                                                                .icon(Signal::from(i::LuExternalLink))
                                                                                .width("1.25em")
                                                                                .height("1.25em")
                                                                                .build(),
                                                                        )
                                                                    ))))
                                                            .build()),
                                                )),
                                                h3().class("text-xl font-bold text-white mb-3 group-hover:text-[#ffef5c] transition-colors duration-300 leading-tight pr-16").child(r.title),
                                                p().class("text-gray-300 leading-relaxed text-sm").child(r.description)
                                            )),
                                            div().class("p-8 relative z-20").child((
                                                div().class("mb-6").child((
                                                    h4().class("text-[#ffef5c] font-semibold text-xs uppercase tracking-wider mb-3 flex items-center gap-2").child((
                                                        div().class("w-1 h-1 bg-[#ffef5c] rounded-full"),
                                                        span().child("Technologies")
                                                    )),
                                                    div().class("flex flex-wrap gap-2").child(
                                                        For(
                                                            ForProps::builder()
                                                                .each(move || r.tech_stack.clone().into_iter().enumerate())
                                                                .key(|(_, t)| t.to_owned())
                                                                .children(move |(idx, t)| {
                                                                    let p = r.teck_stack_percentage[idx];

                                                                    if p >= 80 {
                                                                        span().class("px-3 py-1.5 rounded-lg text-xs font-medium transition-all duration-300 bg-[#ffef5c]/15 text-[#ffef5c] border border-[#ffef5c]/25").child(t)
                                                                    } else if p >=60 {
                                                                        span().class("px-3 py-1.5 rounded-lg text-xs font-medium transition-all duration-300 bg-blue-500/15 text-blue-300 border border-blue-500/25").child(t)
                                                                    } else {
                                                                        span().class("px-3 py-1.5 rounded-lg text-xs font-medium transition-all duration-300 bg-white/8 text-gray-300 border border-white/15").child(t)
                                                                    }
                                                                })
                                                                .build()
                                                        )
                                                    )
                                                )),
                                                div().child((
                                                    h4().class("text-gray-400 font-semibold text-xs uppercase tracking-wider mb-3 flex items-center gap-2").child((
                                                        div().class("w-1 h-1 bg-gray-400 rounded-full"),
                                                        span().child("Tags")
                                                    )),
                                                    div().class("flex flex-wrap gap-1.5").child(
                                                        For(
                                                            ForProps::builder()
                                                                .each(move || r.tags.clone())
                                                                .key(|t| t.to_owned())
                                                                .children(|t| span().class("px-2 py-1 text-xs bg-white/5 text-gray-400 rounded border border-white/10 hover:border-[#ffef5c]/20 hover:text-[#ffef5c] transition-colors duration-300")
                                                                    .child(format!("#{}", t)))
                                                                .build()
                                                        )
                                                    )
                                                )),
                                            )),
                                            div().class("absolute inset-0 bg-gradient-to-br from-[#ffef5c]/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none"),
                                            div().class("absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-[#ffef5c] to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"),
                                        ))
                                    })
                                    .build()
                            )
                        }))
                        .build()
                )
        )
    ))
}
