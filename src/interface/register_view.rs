use dioxus::prelude::*;

use crate::emulator::EmulatorState;

#[component]
#[allow(non_snake_case)]
pub fn RegisterView(emulator_state: Signal<EmulatorState>) -> Element {
    let abi_names = &[
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0/fp", "s1", "a0", "a1", "a2", "a3",
        "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
        "t3", "t4", "t5", "t6",
    ];
    let register_vals = &emulator_state.read().x.x;
    let pc = emulator_state.read().pipeline.datapath.instr_addr_o;
    rsx! {
        div { class: "flex justify-between items-center mb-2",
            h1 { class: "text-lg font-mono font-bold text-gray-900", "Registers" }
            div { class: "bg-white rounded px-3 py-1 shadow-sm",
                span { class: "font-mono font-bold text-gray-700", "PC: " }
                span { class: "font-mono text-blue-600", "{pc:#010x}" }
            }
        }
        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-2",
            for c in 0..4 {
                div { class: "bg-white rounded shadow-sm p-2",
                    div { class: "grid gap-1",
                        for i in (8 * c)..(8 * (c + 1)) {
                            div { class: "flex justify-between items-center border-b border-gray-100 py-1",
                                div { class: "flex-1",
                                    div { class: "font-mono text-gray-500 text-xs", "x{i} ({abi_names[i]})" }
                                    div { class: "font-mono font-bold", "{register_vals[i]:#010x}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}