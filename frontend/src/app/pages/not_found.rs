use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "404 Not Found" }
            p { "Path: {segments:?}" }
        }
    }
}
