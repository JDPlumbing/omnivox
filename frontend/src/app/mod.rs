use dioxus::prelude::*;
use dioxus_router::{Routable, Router};
use crate::app::pages::Home;
use crate::app::pages::Login;
use crate::app::pages::NotFound;

mod pages;

#[derive(Routable, Clone, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
