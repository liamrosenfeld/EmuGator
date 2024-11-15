mod register_view;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use register_view::RegisterView;
use std::ops::Deref;

use crate::{
    assembler::{self, AssembledProgram},
    code_editor::CodeEditor,
    emulator::{self, EmulatorState},
    include_test_file,
};

#[component]
#[allow(non_snake_case)]
pub fn App() -> Element {
    let source = use_signal(|| include_test_file!("syntax-check.s").to_string());
    let mut assembled_program: Signal<Option<AssembledProgram>> = use_signal(|| None);
    let mut emulator_state: Signal<EmulatorState> = use_signal(|| EmulatorState::default());

    use_effect(move || {
        info!("source changed:\n{}", source.read());
        // TODO: Get diagnostics
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
                    button {
                        class: "bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| {
                            match assembler::assemble(&source.read()) {
                                Ok(assembled) => {
                                    emulator_state.set(EmulatorState::default());
                                    assembled_program.set(Some(assembled));
                                }
                                Err(e) => {
                                    info!("Error assembling program: {}", e);
                                }
                            }
                        }, "Start"
                    }
                    if assembled_program.read().is_some() {
                        button {
                            class: "bg-purple-500 hover:bg-purple-600 text-white font-bold py-2 px-4 rounded",
                            onclick: move |_| {
                                if let Some(mut program) = assembled_program.as_mut() {
                                    let new_state = emulator::clock(emulator_state.read().deref(), &mut *program);
                                    *(emulator_state.write()) = new_state;
                                }
                            }, "Next"
                        }
                    }
                    "{emulator_state:?}",
                }
                div {
                    class: "h-1/3 bg-gray-300 p-4",
                    RegisterView {
                        emulator_state: emulator_state
                    }
                }
                div {
                    class: "h-1/3 bg-gray-400 p-4",
                    "{assembled_program:?}",
                }
            }
        }
    }
}