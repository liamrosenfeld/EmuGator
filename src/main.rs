#[allow(non_snake_case, unused)]
mod assembler;
mod code_editor;
mod emulator;
mod isa;
mod utils;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use code_editor::CodeEditor;

const _TAILWIND_URL: &str = manganis::mg!(file("./assets/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    code_editor::register_riscv_language();
    launch(App);
}

#[component]
#[allow(non_snake_case)]
fn App() -> Element {
    let source = use_signal(|| include_test_file!("syntax-check.s").to_string());

    use_effect(move || {
        info!("source changed:\n{}", source.read());
    });

    rsx! {
        div {
            class: "flex h-screen w-full",
            div {
                class: "w-1/2 p-4",
                style: "background-color: #1E1E1E",
                CodeEditor {
                    source: source
                }
            }
            div {
                class: "w-1/2 flex flex-col",
                div {
                    class: "h-1/3 bg-gray-200 p-4",
                    "Right Top View"
                    TestComponent {

                    }
                }
                div {
                    class: "h-1/3 bg-gray-300 p-4",
                    "Right Middle View"
                }
                div {
                    class: "h-1/3 bg-gray-400 p-4",
                    "Right Bottom View"
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn TestComponent() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 {
            class: "text-3xl font-bold mb-6 text-blue-600",
            "Test counter: {count}"
        }
        div {
            class: "space-x-2",
            button {
                class: "bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded mb-2",
                onclick: move |_| count += 1, "Up!"
            }
            button {
                class: "bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded mb-2",
                onclick: move |_| count -= 1, "Down!"
            }
            button {
                class: "bg-purple-500 hover:bg-purple-600 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| testing_function(), "Test Function Call"
            }
        }
    }
}

fn testing_function() {
    let test = 5;
    info!("button pressed! {}", test);
}
