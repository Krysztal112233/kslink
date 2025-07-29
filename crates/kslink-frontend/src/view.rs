use dioxus::prelude::*;

use crate::component::UrlInputBox;

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
