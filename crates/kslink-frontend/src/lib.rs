use crate::view::{Home, PageNotFound, Statistics};

use dioxus::prelude::*;
use dioxus_logger::tracing::info;

mod common;
mod component;
mod request;
mod view;

#[derive(Debug, Clone, Routable, PartialEq, PartialOrd)]
#[rustfmt::skip]
enum Route {
    #[layout(BaseLayout)]
    #[route("/")]
    Home {},

    #[route("/statistics")]
    Statistics {},

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    info!("base url: {}", common::BASE_URL);
    info!("build time: {}", common::BUILD_TIME);

    rsx! {
        div { class: "bg-primary duration-150",
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            Router::<Route> {}
        }
    }
}

#[component]
fn BaseLayout() -> Element {
    rsx! {
        div {
            component::NavBar {
                title: "KSLink",
                links: vec![
                    (Route::Home{}.into(), String::from("Home")),
                    (Route::Statistics{}.into(), String::from("Statistics")),
                ]
            },
        },
        div { class:"z-10",
            Outlet::<Route> {}
        },
        div { }
    }
}
