use icondata as i;
use leptos::{
    html::{a, div},
    prelude::*,
};
use leptos_icons::{Icon, IconProps};

pub fn component() -> impl IntoView {
    div().class("flex flex-row gap-3 items-center h-10").child((
        a().href("https://github.com/rust-dd/")
            .rel("noopener noreferrer")
            .target("_blank")
            .aria_label("GitHub")
            .class("transition-all text-white duration-500 size-6 hover:text-[#ffef5c]")
            .child(Icon(
                IconProps::builder()
                    .icon(Signal::from(i::IoLogoGithub))
                    .width("1.5em")
                    .height("1.5em")
                    .build(),
            )),
        a().href("https://x.com/rust_dd")
            .rel("noopener noreferrer")
            .target("_blank")
            .aria_label("X")
            .class("transition-all text-white duration-500 size-6 hover:text-[#ffef5c]")
            .child(Icon(
                IconProps::builder()
                    .icon(Signal::from(i::FaXTwitterBrands))
                    .width("1.5em")
                    .height("1.5em")
                    .build(),
            )),
        a().href("https://www.linkedin.com/company/rust-dd")
            .rel("noopener noreferrer")
            .target("_blank")
            .aria_label("GitHub")
            .class("transition-all text-white duration-500 size-6 hover:text-[#ffef5c]")
            .child(Icon(
                IconProps::builder()
                    .icon(Signal::from(i::IoLogoLinkedin))
                    .width("1.5em")
                    .height("1.5em")
                    .build(),
            )),
        a().href("/rss.xml")
            .rel("noopener noreferrer")
            .target("_blank")
            .aria_label("GitHub")
            .class("transition-all text-white duration-500 size-6 hover:text-[#ffef5c]")
            .child(Icon(
                IconProps::builder()
                    .icon(Signal::from(i::IoLogoRss))
                    .width("1.5em")
                    .height("1.5em")
                    .build(),
            )),
    ))
}
