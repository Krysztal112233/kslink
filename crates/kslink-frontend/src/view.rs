use dioxus::prelude::*;

use crate::component::{StatisticsItem, StatisticsList, UrlInputBox};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "hero bg-base-200 min-h-screen",
            div {
                class: "hero-content text-center text-base-content",
                div {
                    class: "max-w-max",
                    h1 { class: "text-5xl font-bold m-12",
                        "Simply shorten your link"
                    }
                    UrlInputBox {  },
                }
            }
        }
    }
}

#[component]
pub fn Statistics() -> Element {
    let vec: Vec<(String, String, String, Element)> = vec![(
        "Recorded".to_owned(),
        "100000".to_owned(),
        "links into database".to_owned(),
        rsx! {
            svg {
                class: "size-6",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        },
    )];

    rsx! {
        div {
            class: "hero bg-base-200 min-h-screen",
            div {
                class: "hero-content text-base-content flex-col py-12",
                div {
                    class: "max-w-max text-center",
                    h1 { class: "text-5xl font-bold m-12", "Statistics of this short link service" }
                }
                StatisticsList {
                    for (title, value, desc , icon) in vec {
                        StatisticsItem {
                            title: title,
                            value: value,
                            icon: icon,
                            desc: desc,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "hero bg-base-200 min-h-screen",
            div {
                class: "hero-content text-center text-base-content",
                div {
                    class: "max-w-max",
                    h1 { class: "text-5xl font-bold m-12",
                        "Oops! Page not found!"
                    }
                    p { class: "py-6",
                        "You've come to a wasteland at {route:?}!"
                    }
                }
            }
        }
    }
}
