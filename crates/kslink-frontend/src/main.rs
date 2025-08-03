use dioxus_logger::tracing::Level;
use kslink_frontend::App;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}
