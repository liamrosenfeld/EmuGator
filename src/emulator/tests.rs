#![allow(non_snake_case)]
use dioxus::html::u;

use crate::{emulator, isa::{Operands, ISA}};

use super::*;

fn write(map: &mut BTreeMap<u32, u8>, address: u32, bytes: &[u8]) {
    for (i, &byte) in bytes.iter().enumerate() {
        map.insert(address + i as u32, byte);
    }
}

fn populate(instructions: &[Instruction]) -> AssembledProgram {
    populate_with_offset(instructions, 0)
}

fn populate_with_offset(instructions: &[Instruction], offset: u32) -> AssembledProgram {
    let mut program = AssembledProgram::new();
    for (i, &instruction) in instructions.iter().enumerate() {
        write(
            &mut program.instruction_memory,
            offset + (4 * i) as u32,
            &instruction.raw().to_le_bytes(),
        );
    }
    program
}

#[test]
fn test_LUI() {
    let mut emulator_state = EmulatorState::default();

    // LUI ( x1 := 0x12345000)
    let mut program = populate(&[
        ISA::LUI.build(Operands {
            rd: 1,
            imm: 0x12345000,
            ..Default::default()
        }),
        ISA::LUI.build(Operands {
            rd: 0,
            imm: 0x12345000,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // After LUI, x1 should be loaded with the upper 20 bits of the immediate
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 0x12345000);
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[0], 0x0);
}

#[test]
fn test_AUIPC() {
    let mut emulator_state = EmulatorState::default();

    // AUIPC ( x1 := PC + 0x12345000)
    let mut program = populate(&[ISA::AUIPC.build(Operands {
        rd: 1,
        imm: 0x12345000,
        ..Default::default()
    })]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // After AUIPC, x1 should hold the value (PC + 0x12345000)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(
        emulator_state.x[1],
        emulator_state.pipeline.ID_pc + 0x12345000
    );
}

#[test]
fn test_JAL() {
    let mut emulator_state = EmulatorState::default();

    // JAL ( x1 := PC + 4, jump to PC + 0x100)
    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 0,
            rs1: 0,
            imm: 0,
            ..Default::default()
        }),
        ISA::JAL.build(Operands {
            rd: 1,
            imm: 0x8,
            ..Default::default()
        }),
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }),
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 2,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // NOOP
    emulator_state = clock(&emulator_state, &mut program);

    // After JAL, x1 should contain PC + 4, and the PC should jump to PC + 0x8
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], pc + 4);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc + 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 0);

    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);


}

#[test]
fn test_JAL_neg_offset() {
    let mut emulator_state = EmulatorState::default();

    // JAL ( x1 := PC + 4, jump to PC - 4)
    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 5,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::JAL.build(Operands {
            rd: 1,
            imm: -4,
            ..Default::default()
        }), // JAL (pc = pc - 4)
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    // ADDI ( x5 := x0 + 1)
    emulator_state = clock(&emulator_state, &mut program);
    // ADDI ( x5 := x5 + 1)
    emulator_state = clock(&emulator_state, &mut program);

    // After JAL, x1 should contain PC + 4, and the PC should jump to PC + 0x04
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], pc + 4);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc - 0x04);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    // ADDI ( x5 := x5 + 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 3);
}

#[test]
#[should_panic(expected = "JAL instruction immediate it not on a 4-byte boundary")]
fn test_JAL_panic() {
    let mut emulator_state = EmulatorState::default();

    // JAL ( x1 := PC + 4, jump to PC + 0x122)
    let mut program = populate(&[ISA::JAL.build(Operands {
        rd: 1,
        imm: 0x122,
        ..Default::default()
    })]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // Should panic because the immediate is not on a 4-byte boundary
    emulator_state = clock(&emulator_state, &mut program);
}

#[test]
fn test_JALR() {
    let mut emulator_state = EmulatorState::default();

    // JALR ( x1 := PC + 4, jump to (x2 + 0x4) & ~1)
    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 2,
            rs1: 0,
            imm: 0x4,
            ..Default::default()
        }), // ADDI ( x2 := x0 + 0x100)
        ISA::JALR.build(Operands {
            rd: 1,
            rs1: 2,
            imm: 0x4,
            ..Default::default()
        }), // JALR ( x1 := PC + 4, jump to (x2 + 0x4) & ~1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 2,
            ..Default::default()
        })
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // After ADDI, x2 should be loaded with 0x100
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[2], 0x4);

    // After JALR, x1 should contain PC + 4, and the PC should jump to (x2 + 0x4) & ~1
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], pc + 4);
    assert_eq!(
        emulator_state.pipeline.datapath.instr_addr_o,
        pc + (emulator_state.x[2] + 0x4) & !1
    );

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 0);

    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);
}

#[test]
fn test_JALR_neg_offset() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 2,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 2,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::JALR.build(Operands {
            rd: 1,
            rs1: 2,
            imm: -4,
            ..Default::default()
        }), // JALR ( x1 := PC + 4, jump to (x2 - 4) & ~1)
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    // ADDI ( x5 := x0 + 1)
    emulator_state = clock(&emulator_state, &mut program);
    // ADDI ( x5 := x0 + 1)
    emulator_state = clock(&emulator_state, &mut program);

    // After JALR, x1 should contain PC + 4, and the PC should jump to PC - 4 + 2
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], pc + 4);
    assert_eq!(
        emulator_state.pipeline.datapath.instr_addr_o,
        (pc + emulator_state.x[2] - 4) & !1
    );
}

#[test]
fn test_BEQ() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x1 := x0 + 1)
        ISA::BEQ.build(Operands {
            rs1: 1,
            rs2: 2,
            imm: 0x8,
            ..Default::default()
        }), // BEQ (branch if x1 == x2)
        ISA::BEQ.build(Operands {
            rs1: 0,
            rs2: 2,
            imm: 0x8,
            ..Default::default()
        }), // BEQ (branch if x0 == x2)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 2,
            ..Default::default()
        })
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // ADDI ( x1 := x0 + 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 1);

    // BEQ (branch if x1 == x2) - should not branch because x1 != x2
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.ID_pc, pc + 0x4);

    // BEQ (branch if x0 == x2) - should branch because x0 == x2
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc + 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 0);

    // ADDI ( x5 := x0 + 2)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);
}

#[test]
fn test_BNE() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x1 := x0 + 1)
        ISA::BNE.build(Operands {
            rs1: 0,
            rs2: 2,
            imm: 0x8,
            ..Default::default()
        }), // BNE (branch if x0 != x2)
        ISA::BNE.build(Operands {
            rs1: 1,
            rs2: 2,
            imm: 0x8,
            ..Default::default()
        }), // BNE (branch if x1 != x2)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 2,
            ..Default::default()
        })
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // ADDI ( x1 := x0 + 1)
    emulator_state = clock(&emulator_state, &mut program);

    // BNE (branch if x0 != x2) - should not branch because x0 == x2
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.ID_pc, pc + 0x4);

    // BNE (branch if x1 != x2) - should branch because x1 != x2
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc + 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 0);

    // ADDI ( x5 := x0 + 2)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);
}

#[test]
fn test_BLT() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: -1,
            ..Default::default()
        }), // ADDI ( x1 := x0 - 1)
        ISA::BLT.build(Operands {
            rs1: 0,
            rs2: 1,
            imm: 0x8,
            ..Default::default()
        }), // BLT (branch if x0 < x1)
        ISA::BLT.build(Operands {
            rs1: 1,
            rs2: 0,
            imm: 0x8,
            ..Default::default()
        }), // BLT (branch if x1 < x0)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 2,
            ..Default::default()
        }) // ADDI ( x5 := x0 + 2)
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // ADDI ( x1 := x0 - 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], u32::MAX);

    // BLT (branch if x0 < x1) - should not branch because x0 > x1
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.ID_pc, pc + 0x4);

    // BLT (branch if x1 < x0) - should branch because x1 < x0
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc + 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 0);

    // ADDI ( x5 := x0 + 2)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);
}

#[test]
fn test_BGE() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: -1,
            ..Default::default()
        }), // ADDI ( x1 := x0 - 1)
        ISA::BGE.build(Operands {
            rs1: 1,
            rs2: 0,
            imm: 0x8,
            ..Default::default()
        }), // BGE (branch if x1 >= x0)
        ISA::BGE.build(Operands {
            rs1: 0,
            rs2: 1,
            imm: 0x8,
            ..Default::default()
        }), // BGE (branch if x0 >= x1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 2,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 2)
        ISA::BGE.build(Operands {
            rs1: 0,
            rs2: 2,
            imm: -0x8,
            ..Default::default()
        }) // BGE (branch if x0 >= x2)
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // ADDI ( x1 := x0 - 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], u32::MAX);

    // BGE (branch if x1 >= x0) - should not branch because x0 > x1
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.ID_pc, pc + 0x4);

    // BLT (branch if x0 >= x1) - should branch because x1 < x0
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc + 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 0);

    // ADDI ( x5 := x0 + 2)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);

    // BGE (branch if x0 >= x2) - should branch because x0 == x2
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc - 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);

    // ADDI ( x5 := x0 + 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 1);
}

#[test]
fn test_BLTU() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: u32::MAX as i32,
            ..Default::default()
        }), // ADDI ( x1 := x0 - 1)
        ISA::BLTU.build(Operands {
            rs1: 1,
            rs2: 0,
            imm: 0x8,
            ..Default::default()
        }), // BLTU (branch if x1 < x0)
        ISA::BLTU.build(Operands {
            rs1: 0,
            rs2: 1,
            imm: 0x8,
            ..Default::default()
        }), // BLTU (branch if x0 < x1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 2,
            ..Default::default()
        }) // ADDI ( x5 := x0 + 2)
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // ADDI ( x1 := x0 - 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], u32::MAX);

    // BLTU (branch if x1 < x0) - should not branch because x1 > x0
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.ID_pc, pc + 0x4);

    // BLTU (branch if x0 < x1) - should branch because x0 < x1
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc + 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 0);

    // ADDI ( x5 := x0 + 2)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);
}

#[test]
fn test_BGEU() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: u32::MAX as i32,
            ..Default::default()
        }), // ADDI ( x1 := x0 - 1)
        ISA::BGEU.build(Operands {
            rs1: 0,
            rs2: 1,
            imm: 0x8,
            ..Default::default()
        }), // BGEU (branch if x0 >= x1)
        ISA::BGEU.build(Operands {
            rs1: 1,
            rs2: 0,
            imm: 0x8,
            ..Default::default()
        }), // BGEU (branch if x1 >= x0)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 1)
        ISA::ADDI.build(Operands {
            rd: 5,
            rs1: 0,
            imm: 2,
            ..Default::default()
        }), // ADDI ( x5 := x0 + 2)
        ISA::BGEU.build(Operands {
            rs1: 0,
            rs2: 2,
            imm: -0x8,
            ..Default::default()
        }) // BGEU (branch if x0 >= x2)
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // ADDI ( x1 := x0 - 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], u32::MAX);

    // BGEU (branch if x0 >= x1) - should not branch because x0 < x1
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.ID_pc, pc + 0x4);

    // BLT (branch if x1 >= x0) - should branch because x1 > x0
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc + 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 0);

    // ADDI ( x5 := x0 + 2)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);

    // BGEU (branch if x0 >= x2) - should branch because x0 == x2
    let pc = emulator_state.pipeline.ID_pc;
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.pipeline.datapath.instr_addr_o, pc - 0x8);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 2);

    // ADDI ( x5 := x0 + 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[5], 1);
}

#[test]
fn test_ADDI() {
    let mut emulator_state = EmulatorState::default();

    // ADDI ( x1 := x0 + 1)
    // ADDI ( x1 := x1 + (-1))
    // ADDI ( x0 := x0 + 1 )
    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }),
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 1,
            imm: -1,
            ..Default::default()
        }),
        ISA::ADDI.build(Operands {
            rd: 0,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // ADDI ( x1 := x0 + 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 1);
    // ADDI ( x1 := x1 + 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 0);
    // ADDI ( x0 := x0 + 1) <= special case should be a noop
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[0], 0);
}

#[test]
fn test_SLTI() {
    let mut emulator_state = EmulatorState::default();

    // SLTI ( x1 := x0 < 1)
    // SLTI ( x1 := x1 < (-1))
    // SLTI ( x0 := x0 < 1 )

    let mut program = populate(&[
        ISA::SLTI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }),
        ISA::SLTI.build(Operands {
            rd: 1,
            rs1: 1,
            imm: -1,
            ..Default::default()
        }),
        ISA::SLTI.build(Operands {
            rd: 0,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // SLTI ( x1 := x0 < 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 1);
    // SLTI ( x1 := x1 < (-1))
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 0);
    // SLTI ( x0 := x0 < 1 ) <= Should not change x0
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[0], 0);
}

#[test]
fn test_SLTIU() {
    let mut emulator_state = EmulatorState::default();

    // SLTIU ( x1 := x0 < 1)
    // SLTIU ( x1 := x1 < (-1))
    // SLTIU ( x0 := x0 < 1 )

    let mut program = populate(&[
        ISA::SLTIU.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }),
        ISA::SLTIU.build(Operands {
            rd: 1,
            rs1: 1,
            imm: -1, // Should be treated as an unsigned number (pretty large)
            ..Default::default()
        }),
        ISA::SLTIU.build(Operands {
            rd: 0,
            rs1: 0,
            imm: 1,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // SLTI ( x1 := x0 < 1)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 1);
    // SLTI ( x1 := x1 < (-1))
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 1);
    // SLTI ( x0 := x0 < 1 ) <= Should not change x0
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[0], 0);
}

#[test]
fn test_XORI() {
    let mut emulator_state = EmulatorState::default();

    // XORI ( x1 := x0 ^ 4)
    // XORI ( x1 := x1 ^ (-1))
    // XORI ( x0 := x0 ^ 100 )

    let mut program = populate(&[
        ISA::XORI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 4,
            ..Default::default()
        }),
        ISA::XORI.build(Operands {
            rd: 1,
            rs1: 1,
            imm: -1,
            ..Default::default()
        }),
        ISA::XORI.build(Operands {
            rd: 0,
            rs1: 0,
            imm: 100,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // XORI ( x1 := x0 ^ 4)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 4);
    // XORI ( x1 := x1 ^ (-1))
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1] as i32, -5);
    // XORI ( x0 := x0 ^ 100 ) <= Should not change x0
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[0], 0);
}

#[test]
fn test_ORI() {
    let mut emulator_state = EmulatorState::default();

    // ORI ( x1 := x0 | 12)
    // ORI ( x1 := x1 | (-1))
    // ORI ( x0 := x0 | 100 )

    let mut program = populate(&[
        ISA::ORI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 12,
            ..Default::default()
        }),
        ISA::ORI.build(Operands {
            rd: 1,
            rs1: 1,
            imm: -10,
            ..Default::default()
        }),
        ISA::ORI.build(Operands {
            rd: 0,
            rs1: 0,
            imm: 100,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // ORI ( x1 := x0 | 12)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 12);
    // ORI ( x1 := x1 ^ (-10))
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1] as i32, -2);
    // ORI ( x0 := x0 ^ 100 ) <= Should not change x0
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[0], 0);
}

#[test]
fn test_ANDI() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 37,
            ..Default::default()
        }),
        ISA::ANDI.build(Operands {
            rd: 1,
            rs1: 1,
            imm: 5,
            ..Default::default()
        }),
        ISA::ANDI.build(Operands {
            rd: 1,
            rs1: 1,
            imm: -10,
            ..Default::default()
        }),
        ISA::ANDI.build(Operands {
            rd: 0,
            rs1: 0,
            imm: 100,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // Set x1 := 37
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 37);

    // ANDI ( x1 := x1 & 5)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 5);

    // ANDI ( x1 := x1 & (-10))
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 4);

    // ANDI ( x0 := x0 & 100 ) <= Should not change x0
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[0], 0);
}

#[test]
fn test_SLLI() {
    let mut emulator_state = EmulatorState::default();

    let mut program = populate(&[
        ISA::ADDI.build(Operands {
            rd: 1,
            rs1: 0,
            imm: 10,
            ..Default::default()
        }),
        ISA::SLLI.build(Operands {
            rd: 2,
            rs1: 1,
            imm: 4,
            ..Default::default()
        }),
        ISA::SLLI.build(Operands {
            rd: 3,
            rs1: 1,
            imm: 0b100001,
            ..Default::default()
        }),
        ISA::SLLI.build(Operands {
            rd: 0,
            rs1: 0,
            imm: 3,
            ..Default::default()
        }),
    ]);

    // Instruction fetch
    emulator_state = clock(&emulator_state, &mut program);

    // Set x1 := 10
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[1], 10);

    // SLLI ( x2 := x1 << 4)
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[2], 160);

    // SLLI ( x3 := x1 << 0b1000001) Should only shift 1 time since we only look at last 5 bits
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[3], 20);

    // SLLI ( x0 := x1 << 3 ) <= Should not change x0
    emulator_state = clock(&emulator_state, &mut program);
    assert_eq!(emulator_state.x[0], 0);
}
