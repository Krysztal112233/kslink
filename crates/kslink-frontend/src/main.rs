use crate::view::Home;
use crate::view::Statistics;
use dioxus::prelude::*;

mod component;
mod view;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(BaseLayout)]
    #[route("/")]
    Home {},
    #[route("/statistics")]
     Statistics {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
