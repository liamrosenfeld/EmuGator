mod datapath;
mod isa;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use datapath::CVE2Pipeline;
use isa::{XLEN, Instruction, get_handler};

macro_rules! bits {
    ( $val:expr,$start_bit:expr,$width:expr ) => {
        { ($val >> $start_bit) & (2^$width - 1) }
    };
    ( $val:expr,$end_bit:expr;$start_bit:expr ) => {
        bits!($val,$start_bit,$end_bit-$start_bit+1)
    };
    ( $val:expr,$bit:expr ) => {
        bits!($val,$bit,1)
    }
}
pub(crate) use bits;  

pub type InstructionHandler = fn(&Instruction, &mut EmulatorState);

#[derive(Copy, Clone)]
pub struct EmulatorState {
    pub pc: XLEN,
    pub x: [XLEN; 32],
    pub pipeline: CVE2Pipeline,
}

pub struct Emulator {
    states: Vec<EmulatorState>
}

impl Emulator {
    pub fn clock(&mut self, instruction_map: &BTreeMap<u32, u8>, data_map: &mut BTreeMap<u32, u8>) {
        // Get a mutable copy of the last state
        let mut state = *self.states.last().unwrap();
        
        // Load the fetched instruction into the instr_rdata lines
        if state.pipeline.datapath.instr_req_o {
            let instr_addr = state.pipeline.datapath.instr_addr_o;

            // Read the next instruction into the instruction fetch register
            let mut instr_bytes: [u8; 4] = [0; 4];
            let success = (0usize..4usize).all(|i| {
                let addr = instr_addr + i as u32;
                let valid = instruction_map.contains_key(&addr);
                
                if valid {
                    instr_bytes[i] = instruction_map[&addr];
                }
                valid
            });

            // Write the read dada to the instruction read data lines
            state.pipeline.datapath.instr_rdata_i = u32::from_le_bytes(instr_bytes);

            // Set the appropriate flags 
            if success {
                state.pipeline.datapath.instr_gnt_i = true;
                state.pipeline.datapath.instr_rvalid_i = true;
                state.pipeline.datapath.instr_err_i = false;
            } else {
                state.pipeline.datapath.instr_gnt_i = true;
                state.pipeline.datapath.instr_rvalid_i = false;
                state.pipeline.datapath.instr_err_i = true;
            }
        }

        // Decode the instruction in the instruction decode register
        let instr = Instruction{ instr: state.pipeline.ID };
        match get_handler(instr) {
            Err(()) => println!("Invalid Instruction {}", instr.instr),
            Ok(handler) => handler(&instr, &mut state),
        };

        // Perform any requested memory read/write
        if state.pipeline.datapath.data_req_o {
            let data_addr = state.pipeline.datapath.data_addr_o;
            let data_we = state.pipeline.datapath.data_we_o;
            let data_be = state.pipeline.datapath.data_be_o;
            let data_wdata = state.pipeline.datapath.data_wdata_o;

            let mut data_bytes: [u8; 4] = [0; 4];
            let success = (0usize..4usize).all(|i| {
                let addr = data_addr + i as u32;
                let valid = instruction_map.contains_key(&addr);
                
                if valid {
                    // Read byte
                    data_bytes[i] = data_map[&addr];
                    // If we are writing then write the byte
                    if data_we && bits!(data_be, i) != 0 {
                        data_map.insert(addr, bits!(data_wdata, i*8, 8) as u8);
                    } 
                }
                valid
            });

            state.pipeline.datapath.data_rdata_i = u32::from_le_bytes(data_bytes);

            // Set the appropriate flags 
            if success {
                state.pipeline.datapath.data_gnt_i = true;
                state.pipeline.datapath.data_rvalid_i = true;
                state.pipeline.datapath.data_err_i = false;
            } else {
                state.pipeline.datapath.data_gnt_i = true;
                state.pipeline.datapath.data_rvalid_i = false;
                state.pipeline.datapath.data_err_i = true;
            }
        }

        self.states.push(state);
    }
}
