use dioxus::prelude::*;

pub fn App() -> Element {
    // ⬅ state/signals come first, before rendering
    let mut count = use_signal(|| 0);

    rsx! {
        // ⬅ global style block at the top of your UI tree
        style { include_str!("./styles/index.css") }

        div {
            h1 { "Omnivox" }
            p { "Landing page placeholder — wiring in progress." }

            button {
                onclick: move |_| count += 1,
                "Clicked {count} times"
            }
        }
    }
}
