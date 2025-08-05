use dioxus::prelude::*;

use crate::{
    component::{StatisticsItem, StatisticsList, UrlInputBox},
    request,
};

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
    let request = use_resource(|| async move {
        let data = request::Requester::new()
            .statistics()
            .await
            .unwrap_or_default();

        let vec = vec![
            ("Shorted", data.count, "links in total"),
            ("Served", data.visit, "visitors"),
        ];

        vec
    });

    rsx! {
        div {
            class: "hero bg-base-200 min-h-screen",
            div {
                class: "hero-content text-base-content flex-col py-12",
                div {
                    class: "max-w-max text-center",
                    h1 { class: "text-5xl font-bold m-12", "Statistics of this short link service" }
                }
                match &*request.read_unchecked() {
                    Some(data) => rsx! {
                        StatisticsList { class: "w-auto",
                            for (title, value, subtitle) in data {
                                StatisticsItem {
                                    title: title,
                                    value: value,
                                    subtitle: subtitle,
                                }
                            }
                        }
                    },
                    None => rsx!{ div { class: "skeleton w-120 h-30" } },
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
