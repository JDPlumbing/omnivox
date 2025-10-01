use dioxus::prelude::*;

pub fn Login() -> Element {
    rsx! {
        div {
            h1 { "Login" }
            input { r#type: "text", placeholder: "Username" }
            input { r#type: "password", placeholder: "Password" }
            button { "Submit" }
        }
    }
}
