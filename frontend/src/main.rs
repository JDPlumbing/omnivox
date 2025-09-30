use dioxus::prelude::*;
use dioxus_web::launch; // <-- import the function

mod app;

fn main() {
    launch(app::App);
}

fn app() -> Element {
    rsx! {
        style { "{include_str!(\"./styles/index.css\")}" }
        div { class: "flex items-center justify-center min-h-screen bg-gray-100",
            div { class: "text-center p-8 bg-white rounded-lg shadow-lg",
                h1 { class: "text-3xl font-bold text-gray-800 mb-4", "Welcome to Omnivox" }
                p { class: "text-gray-600 mb-6", "Your landing page is alive and well." }
                button { class: "px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition",
                    "Click Me"
                }
            }
        }
    }
}