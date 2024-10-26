use super::isa::{XLEN, Instruction, get_handler};

pub type InstructionHandler = fn(Instruction, EmulatorState) -> EmulatorState;

#[derive(Copy, Clone)]
pub struct EmulatorState {
    pc: XLEN,
    x: [XLEN; 32],
}

pub struct Emulator {
    states: Vec<EmulatorState>
}

impl Emulator {
    pub fn decode(&mut self, instruction: Instruction) {
        self.states.push(
            match get_handler(instruction) {
                Err(()) => { 
                    let last_state  = *self.states.last().unwrap();
                    println!("Invalid Instruction at {}", last_state.pc);
                    last_state
                },
                Ok(handler) => handler(instruction, *self.states.last().unwrap())                
            }
        )
    }
}
