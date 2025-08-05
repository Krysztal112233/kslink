use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_logger::tracing;
use url::Url;

use crate::{Route, common, request::Requester};

#[component]
fn NavBarTitle(title: String) -> Element {
    rsx! {
        div { class: "flex-1",
            button { class: "btn btn-ghost text-xl",
                Link { to: Route::Home {  }, "{title}" }
            }
        }
    }
}

#[component]
fn NavBarLinks(to: NavigationTarget, title: String) -> Element {
    rsx! { li { Link { class: "text-xl", to: to, "{title}" } } }
}

#[component]
pub fn UrlInputBox() -> Element {
    let mut url = use_signal(String::new);
    let mut is_working = use_signal(|| false);
    let mut short_url = use_signal(|| None::<String>);

    let is_valid = url.with(|u| common::is_valid_url(u));
    let input_class = format!(
        "input join-item {}",
        if is_valid {
            "input-success"
        } else {
            "input-error"
        }
    );
    let btn_class = format!(
        "btn join-item btn-secondary hover:btn-primary {}",
        if !is_valid || is_working() {
            "btn-disabled"
        } else {
            ""
        }
    );

    let on_input = move |event: Event<FormData>| {
        short_url.set(None);
        url.set(event.value());
    };

    let on_submit = move |_: Event<MouseData>| async move {
        if !is_valid || is_working() {
            return;
        }

        is_working.set(true);

        if let Ok(url) = Url::from_str(&url()) {
            tracing::info!("Creating short link for url {url}");
            if let Ok(result) = Requester::new().create(url).await {
                short_url.set(Some(format!("{}/{result}", common::BASE_URL)));
            }
        }

        is_working.set(false);
    };

    rsx! {
        div {
            div { class: "join pt-px-8",
                input {
                    class: input_class,
                    placeholder: "https://...",
                    oninput: on_input,
                    value: "{url}",
                },
                button {
                    class: btn_class,
                    onclick: on_submit,
                    disabled: !is_valid || is_working(),
                    if !is_working() {
                        "Make it shorter!"
                    } else {
                        span { class: "loading loading-spinner loading-md" }
                    }
                }
            }

            if let Some(url) = short_url() {
                div { class: "alert alert-success mt-4",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "stroke-current shrink-0 h-6 w-6",
                        fill: "none",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    span { "Your short URL is: "
                        a {
                            href: "{url}",
                            target: "_blank",
                            class: "link link-hover",
                            "{url}"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Debug, Clone, PartialEq)]
pub struct NavBarProps {
    title: String,
    links: Vec<(NavigationTarget, String)>,
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    rsx! {
        div { class: "fixed top-0 left-1/2 transform -translate-x-1/2 z-50 w-full mt-4 max-w-15/16 rounded-xl navbar bg-base-300 shadow-xl backdrop-blur text-base-content",
            NavBarTitle { title: "{props.title}" },
            div { class: "flex-none",
                ul { class: "menu menu-horizontal px-1",
                    li { Link { class: "text-xl", to: "https://github.com/Krysztal112233/kslink", "GitHub" } }
                    for (to, name) in props.links { NavBarLinks { to: to, title: "{name}" } }
                    ThemeToggle { },
                }
            }
        }
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        label {
            class: "swap swap-rotate",

            input {
                r#type: "checkbox",
                class: "theme-controller",
                value: "dracula"
            },

            svg {
                class: "swap-off h-10 w-10 fill-current",
                view_box: "0 0 24 24",
                path {
                    d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"
                }
            }

            svg {
                class: "swap-on h-10 w-10 fill-current",
                view_box: "0 0 24 24",
                path {
                    d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
                }
            }
        }
    }
}

#[derive(Debug, Props, Clone, PartialEq)]
pub struct StatisticsItemProps {
    pub title: String,
    pub value: String,
    pub desc: Option<String>,
    pub icon: Option<Element>,
}

#[component]
pub fn StatisticsItem(props: StatisticsItemProps) -> Element {
    rsx! {
        div { class: "stat",
            if let Some(icon) = props.icon { div { class: "stat-figure text-secondary", {icon} } }
            div { class: "stat-title text-xl", "{props.title}" }
            div { class: "stat-value", "{props.value}" }
            if let Some(desc) = props.desc { div { class: "stat-desc", "{desc}" } }
        }
    }
}

#[derive(Debug, Props, PartialEq, Clone)]
pub struct StatisticsListProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn StatisticsList(props: StatisticsListProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div { class: "stats hover:shadow-md duration-200 {class}",
            { props.children }
        }
    }
}
