use super::*;
use crate::include_test_file;

#[test]
fn assembler_roundtrip() {
    let simple_loop_program = include_test_file!("syntax-check.s");

    match get_emulator_maps(simple_loop_program) {
        Ok((inst_mem, source_map, data_mem)) => {
            // inst_mem: BTreeMap<u32, u8> - instruction memory
            // source_map: BTreeMap<u32, usize> - source line mapping
            // data_mem: BTreeMap<u32, u8> - data memory

            println!("Instruction Memory (Address -> Byte):");
            for (&addr, &byte) in &inst_mem {
                println!("0x{:08X}: 0x{:02X}", addr, byte);
            }

            println!("\nSource Map (Address -> Line Number):");
            for (&addr, &line) in &source_map {
                println!("0x{:08X}: Line {}", addr, line);
            }

            println!("\nData Memory (Address -> Byte):");
            for (&addr, &byte) in &data_mem {
                println!("0x{:08X}: 0x{:02X}", addr, byte);
            }

            println!("\nReconstructed 32-bit Instructions:");
            for &addr in source_map.keys() {
                let instruction = u32::from_le_bytes([
                    inst_mem[&addr],
                    inst_mem[&(addr + 1)],
                    inst_mem[&(addr + 2)],
                    inst_mem[&(addr + 3)],
                ]);
                println!("0x{:08X}: 0x{:08X}", addr, instruction);
            }
        }
        Err(e) => {
            eprintln!("Assembly error: {}", e);
        }
    }
}
