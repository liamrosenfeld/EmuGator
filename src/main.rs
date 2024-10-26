#![allow(non_snake_case)]
#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]

mod emulator;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 {
                class: "text-3xl font-bold mb-6 text-blue-600",
                "High-Five counter: {count}"
            }
            button {
                class: "bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded mb-2",
                onclick: move |_| count += 1, "Up high!"
            }
            button {
                class: "bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded mb-2",
                onclick: move |_| count -= 1, "Down low!"
            }
            button {
                class: "bg-purple-500 hover:bg-purple-600 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| another_function(), "Another Button"
            }
        }
    }
}

fn another_function() {
    let test = 5;
    info!("button pressed! {}", test);
}
