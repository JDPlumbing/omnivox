use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li { Link { to: "/", "Home" } }
                li { Link { to: "/login", "Login" } }
            }
        }
    }
}
