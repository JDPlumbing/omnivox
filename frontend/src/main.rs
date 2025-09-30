use dioxus::prelude::*;
use dioxus_web::launch;

fn main() {
    launch(app);
}

// In 0.7, components are just plain functions returning `Element`
fn app() -> Element {
    rsx! {
        div {
            h1 { "Hello from Dioxus 0.7 🚀" }
            p { "If you see this, your frontend is alive." }
        }
    }
}
