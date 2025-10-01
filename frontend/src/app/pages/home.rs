use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx! {
        div {
            h1 { "Welcome to Omnivox" }
            p { "Landing page placeholder" }
        }
    }
}
