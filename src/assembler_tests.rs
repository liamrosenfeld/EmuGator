mod assembler;

fn main() {
    // Example assembly program as a string
    let test_program = r#"
.data
message: .string "test\n"
numbers: .word 1, 2, 3, 4
bytes: .byte 0xFF, 0x42, 0x33
array: .ascii "test"

.text
main:
    # I-type instructions
    ADDI x5, x6, 5
    SLTI x5, x6, 5
    SLTIU x5, x6, 5
    ANDI x5, x6, 0xFF
    ORI x5, x6, 0xFF
    XORI x5, x6, 0xFF
    SLLI x5, x6, 2
    SRLI x5, x6, 2
    SRAI x5, x6, 2

    # U-type instructions
    LUI x5, 0xFFF
    AUIPC x5, 0xFFF

    # R-type instructions
    ADD x5, x6, x7
    SUB x5, x6, x7
    SLT x5, x6, x7
    SLTU x5, x6, x7
    AND x5, x6, x7
    OR x5, x6, x7
    XOR x5, x6, x7
    SLL x5, x6, x7
    SRL x5, x6, x7
    SRA x5, x6, x7

    # J-type instruction
    JAL x5, function1

    # More I-type instructions
    JALR x5, x6, 0x100

    # B-type instructions
    BEQ x5, x6, branch_target1
    BNE x5, x6, branch_target2
    BLT x5, x6, branch_target3
    BLTU x5, x6, branch_target4
    BGE x5, x6, branch_target5
    BGEU x5, x6, branch_target6

    # Memory I-type loads
    LW x5, 0(x6)
    LH x5, 0(x6)
    LHU x5, 0(x6)
    LB x5, 0(x6)
    LBU x5, 0(x6)

    # Memory S-type stores
    SW x5, 0(x6)
    SH x5, 0(x6)
    SB x5, 0(x6)

    # Special I-type instructions
    # FENCE
    # ECALL
    # EBREAK

function1:
    JAL x5, function2

function2:
    JAL x5, branch_target1

branch_target1:
    ADDI x5, x6, 10

branch_target2:
    ADDI x5, x6, 10

branch_target3:
    ADDI x5, x6, 10

branch_target4:
    ADDI x5, x6, 10

branch_target5:
    ADDI x5, x6, 10

branch_target6:
    ADDI x5, x6, 10
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