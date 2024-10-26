use super::*;

#[test]
fn assembler_roundtrip() {
    let _test_program = r#"
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
    JAL x5, branch_target2

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

    let _simple_loop_program = r#"
.data
result: .word 6   # Allocate space to store final result
result2: .word 5  # Allocate a second word for fun for this test

.text
main:
    # Initialize counter (x5) to 3
    ADDI x5, x0, 3
    # Initialize x6 to 0
    ADD x6, x0, x0

loop:
    # Add 3 to x6
    ADDI x6, x6, 3
    
    # Decrement counter
    ADDI x5, x5, -1
    
    # If counter >= 0, continue loop
    BGE x5, x0, loop
    
    # Load address of result (we'll need to add the actual address)
    # Store x6 at address in x7
    SW x6, result2    # Store x6 at address in x7
"#;

    match get_emulator_maps(_simple_loop_program) {
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
