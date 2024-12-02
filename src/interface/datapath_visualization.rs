use crate::emulator::EmulatorState;
use dioxus::prelude::*;

#[component]
#[allow(non_snake_case)]
pub fn DatapathVisualization(emulator_state: Signal<EmulatorState>) -> Element {
    let pipeline = emulator_state.read().pipeline;
    rsx! {
        div { class: "w-full h-full overflow-y-scroll",
            div { class: "mb-4 font-mono text-sm whitespace-pre", "{pipeline:#?}" }
        }
    }
}
