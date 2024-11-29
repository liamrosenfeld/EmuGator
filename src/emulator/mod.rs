mod datapath;
mod handlers;

#[cfg(test)]
mod tests;

use crate::assembler::AssembledProgram;
use crate::isa::Instruction;
use crate::{bitmask, bits};
use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

use datapath::CVE2Pipeline;
use handlers::get_handler;

pub type InstructionHandler = fn(&Instruction, &mut EmulatorState);

#[derive(Copy, Clone, Default, Debug)]
pub struct RegisterFile {
    pub x: [u32; 32],
}

impl Index<usize> for RegisterFile {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            return &0;
        } else {
            &self.x[index]
        }
    }
}

impl IndexMut<usize> for RegisterFile {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.x[0] = 0;
        &mut self.x[index]
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct EmulatorState {
    pub x: RegisterFile,
    pub pipeline: CVE2Pipeline,
}

pub fn clock(org_state: &EmulatorState, program: &mut AssembledProgram) -> EmulatorState {
    let mut next_state = *org_state;

    // Load the fetched instruction into the instr_rdata lines
    if next_state.pipeline.datapath.instr_req_o {
        let instr_addr = next_state.pipeline.datapath.instr_addr_o;

        // Read the next instruction into the instruction fetch register
        let mut instr_bytes: [u8; 4] = [0; 4];
        let success = (0usize..4usize).all(|i| {
            let addr = instr_addr + i as u32;
            let valid = program.instruction_memory.contains_key(&addr);

            if valid {
                instr_bytes[i] = program.instruction_memory[&addr];
            }
            valid
        });

        // Write the read data to the instruction read data lines
        next_state.pipeline.datapath.instr_rdata_i = u32::from_le_bytes(instr_bytes);

        // Set the appropriate flags
        if success {
            next_state.pipeline.datapath.instr_gnt_i = true;
            next_state.pipeline.datapath.instr_rvalid_i = true;
            next_state.pipeline.datapath.instr_err_i = false;

            // Store the fetched instruction
            next_state.pipeline.IF = next_state.pipeline.datapath.instr_rdata_i;
            next_state.pipeline.IF_pc = next_state.pipeline.datapath.instr_addr_o;
            // Move the program counter
            next_state.pipeline.datapath.instr_addr_o += 4;
        } else {
            next_state.pipeline.datapath.instr_gnt_i = true;
            next_state.pipeline.datapath.instr_rvalid_i = false;
            next_state.pipeline.datapath.instr_err_i = true;
        }
    }

    // Decode the instruction in the instruction decode register
    let instr = Instruction::from_raw(next_state.pipeline.ID);

    match get_handler(instr) {
        Err(()) => println!("Invalid Instruction {}", instr.raw()),
        Ok(handler) => handler(&instr, &mut next_state),
    };

    // Perform any requested memory read/write
    if next_state.pipeline.datapath.data_req_o {
        let data_addr = next_state.pipeline.datapath.data_addr_o;
        let data_we = next_state.pipeline.datapath.data_we_o;
        let data_be = next_state.pipeline.datapath.data_be_o;
        let data_wdata = next_state.pipeline.datapath.data_wdata_o;

        let mut data_bytes: [u8; 4] = [0; 4];
        let success = (0usize..4usize).all(|i| {
            let addr = data_addr + i as u32;
            let valid = program.instruction_memory.contains_key(&addr);

            if valid {
                // Read byte
                data_bytes[i] = program.data_memory[&addr];
                // If we are writing then write the byte
                if data_we && bits!(data_be, i) != 0 {
                    program.data_memory.insert(addr, bits!(data_wdata, i * 8, 8) as u8);
                }
            }
            valid
        });

        next_state.pipeline.datapath.data_rdata_i = u32::from_le_bytes(data_bytes);

        // Set the appropriate flags
        if success {
            next_state.pipeline.datapath.data_gnt_i = true;
            next_state.pipeline.datapath.data_rvalid_i = true;
            next_state.pipeline.datapath.data_err_i = false;
        } else {
            next_state.pipeline.datapath.data_gnt_i = true;
            next_state.pipeline.datapath.data_rvalid_i = false;
            next_state.pipeline.datapath.data_err_i = true;
        }
    }

    // Only load the next instruction if the fetch is enabled
    if next_state.pipeline.datapath.fetch_enable_i {
        next_state.pipeline.ID = next_state.pipeline.IF;
        next_state.pipeline.ID_pc = next_state.pipeline.IF_pc;
    }
    return next_state;
}
