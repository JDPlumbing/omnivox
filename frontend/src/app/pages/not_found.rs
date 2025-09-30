use dioxus::prelude::*;

pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { "404 - Page not found: {segments:?}" }
    }
}
