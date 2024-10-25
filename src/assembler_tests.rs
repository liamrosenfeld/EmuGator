mod assembler;

fn main() {
    let test_program = r#"
.data
message:
    .string "yay\n"
numbers:
    .word 1, 2, 3, 4
bytes:
    .byte 0xFF, 0x42, 0x33
array:
    .ascii "test"

.text
main:
    # Load the message address into x1
    lw x1, message
    # Load first number into x2
    lw x2, numbers
    # Load first byte into x3
    lb x3, bytes
    # Load array into x4
    lw x4, array

    # Do some operations
    addi x5, x0, 5
    add x6, x2, x5
    sw x6, 0(x1)
"#;

    // Get the maps from the assembler
    match assembler::get_emulator_maps(test_program) {
        Ok((instruction_memory, source_map, data_memory)) => {
            // Print instruction memory
            println!("\nInstruction Memory (Address -> Byte):");
            for (addr, byte) in instruction_memory.iter() {
                println!("0x{:08X}: 0x{:02X}", addr, byte);
            }

            // Print source map (only instruction start addresses)
            println!("\nSource Map (Address -> Line Number):");
            for (addr, line) in source_map.iter() {
                println!("0x{:08X}: Line {}", addr, line);
            }

            // Print data memory
            println!("\nData Memory (Address -> Byte):");
            for (addr, byte) in data_memory.iter() {
                println!("0x{:08X}: 0x{:02X}", addr, byte);
            }

            // Example of reconstructing full 32-bit instructions
            println!("\nReconstructed 32-bit Instructions:");
            let mut addr = 0u32;
            while addr < (instruction_memory.len() as u32) {
                if source_map.contains_key(&addr) {
                    let instruction = 
                        (*instruction_memory.get(&(addr + 3)).unwrap_or(&0) as u32) << 24 |
                        (*instruction_memory.get(&(addr + 2)).unwrap_or(&0) as u32) << 16 |
                        (*instruction_memory.get(&(addr + 1)).unwrap_or(&0) as u32) << 8 |
                        (*instruction_memory.get(&addr).unwrap_or(&0) as u32);
                    
                    println!("0x{:08X}: 0x{:08X}", addr, instruction);
                }
                addr += 4;
            }
        },
        Err(e) => println!("Assembly error: {}", e),
    }
}