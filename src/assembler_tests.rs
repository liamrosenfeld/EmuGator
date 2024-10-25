mod assembler;

fn main() {
    // Example assembly program as a string
    let test_program = r#"
.data
message: .string "yay\n"
numbers: .word 1, 2, 3, 4
bytes: .byte 0xFF, 0x42, 0x33
array: .ascii "test"

.text
main:lw x1, message
    # Load first number into x2
    lw x2, numbers
    # Load first byte into x3
    lb x3, bytes
    # Load array into x4
    lw x4, array

    # Do some operations
    addi x5, x0, 5
    add x6, x2, x5
    sw x6, 0(x6)
"#;

    // Call get_emulator_maps and handle the Result
    match assembler::get_emulator_maps(test_program) {
        Ok((inst_mem, source_map, data_mem)) => {
            // Now you have three local variables:
            // inst_mem: BTreeMap<u32, u8> - instruction memory
            // source_map: BTreeMap<u32, usize> - source line mapping
            // data_mem: BTreeMap<u32, u8> - data memory
            
            // Example: print out the maps
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
                    inst_mem[&(addr + 3)]
                ]);
                println!("0x{:08X}: 0x{:08X}", addr, instruction);
            }
        },
        Err(e) => {
            eprintln!("Assembly error: {}", e);
        }
    }
}