use dioxus::prelude::*;
use crate::assembler::{AssembledProgram, Section};

#[component]
#[allow(non_snake_case)]
pub fn InstructionView(assembled_program: Signal<Option<AssembledProgram>>) -> Element {
    let program = assembled_program.read();
    
    // Early return if no program is assembled
    if program.is_none() {
        return rsx! {
            div { class: "flex justify-center items-center h-full",
                span { class: "text-gray-500 font-mono", "No program loaded" }
            }
        };
    }

    let program = program.as_ref().unwrap();
    let instruction_memory = &program.instruction_memory;
    let text_start = program.get_section_start(Section::Text) as usize;
    
    // Number of columns to display
    let num_columns = 3;
    let total_instructions = instruction_memory.len() / 4; // Since each instruction is 4 bytes

    rsx! {
        div { 
            class: "flex flex-col h-full",
            div { 
                class: "flex justify-between items-center mb-2",
                h1 { class: "text-lg font-mono font-bold text-gray-900", "Instruction Memory" }
            }
            div { 
                class: "grid gap-2 overflow-auto max-h-[calc(100vh-12rem)] pr-2",
                for row in (0..total_instructions).step_by(num_columns) {
                    div { 
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2",
                        for col in 0..num_columns {
                            {
                                let base_addr = text_start + (row + col) * 4;
                                rsx! {
                                    {
                                        if base_addr < text_start + instruction_memory.len() {
                                            rsx! {
                                                div { 
                                                    class: "bg-white rounded shadow-sm p-2",
                                                    div { 
                                                        class: "flex flex-col py-1",
                                                        div { class: "font-mono text-gray-500 text-xs",
                                                            "0x{base_addr:04x}:"
                                                        }
                                                        div { class: "font-mono font-bold pl-4",
                                                            {
                                                                let instruction = (instruction_memory.get(&(base_addr as u32)).copied().unwrap_or(0) as u32) |
                                                                    ((instruction_memory.get(&((base_addr + 1) as u32)).copied().unwrap_or(0) as u32) << 8) |
                                                                    ((instruction_memory.get(&((base_addr + 2) as u32)).copied().unwrap_or(0) as u32) << 16) |
                                                                    ((instruction_memory.get(&((base_addr + 3) as u32)).copied().unwrap_or(0) as u32) << 24);
                                                                
                                                                rsx! {
                                                                    span { class: "font-mono",
                                                                        "0x{instruction:08x}"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if let Some(line) = program.source_map.get(&(base_addr as u32)) {
                                                            div { class: "text-xs text-gray-500 pl-4",
                                                                "Line {line}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            rsx! { div { } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}