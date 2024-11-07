#![allow(non_snake_case)]
use crate::isa::{Operands, ISA};

use super::*;

fn write(map: &mut BTreeMap<u32, u8>, address: u32, bytes: &[u8]) {
    for (i, &byte) in bytes.iter().enumerate() {
        map.insert(address + i as u32, byte);
    }
}

fn populate(map: &mut BTreeMap<u32, u8>, instructions: &[Instruction]) {
    for (i, &instruction) in instructions.iter().enumerate() {
        write(map, (4 * i) as u32, &instruction.raw().to_le_bytes());
    }
}

#[test]
fn test_LUI() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    // LUI ( x1 := 0x12345000)
    populate(
        &mut instruction_map,
        &[
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
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);

    // After LUI, x1 should be loaded with the upper 20 bits of the immediate
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[1], 0x12345000);
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[0], 0x0);
}

#[test]
fn test_AUIPC() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    // AUIPC ( x1 := PC + 0x12345000)
    populate(
        &mut instruction_map,
        &[ISA::AUIPC.build(Operands {
            rd: 1,
            imm: 0x12345000,
            ..Default::default()
        })],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);

    // After AUIPC, x1 should hold the value (PC + 0x12345000)
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(
        emulator.state().x[1],
        emulator.state().pipeline.datapath.instr_addr_o + 0x12345000
    );
}

#[test]
fn test_JAL() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    // JAL ( x1 := PC + 4, jump to PC + 0x100)
    populate(
        &mut instruction_map,
        &[ISA::JAL.build(Operands {
            rd: 1,
            imm: 0x100,
            ..Default::default()
        })],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);

    // After JAL, x1 should contain PC + 4, and the PC should jump to PC + 0x100
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[1], pc + 4);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x100);
}

#[test]
fn test_JALR() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            ISA::JALR.build(Operands {
                rd: 1,
                rs1: 2,
                imm: 0x4,
                ..Default::default()
            }), // JALR ( x1 := PC + 4, jump to (x2 + 0x4) & ~1)
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);

    // After JALR, x1 should contain PC + 4, and the PC should jump to (x2 + 0x4) & ~1
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[1], pc + 4);
    assert_eq!(
        emulator.state().pipeline.datapath.instr_addr_o,
        (pc + emulator.state().x[2] + 0x4) & !1
    );
}

#[test]
fn test_BEQ() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            ISA::ADDI.build(Operands {
                rd: 1,
                rs1: 0,
                imm: 1,
                ..Default::default()
            }), // ADDI ( x1 := x0 + 1)
            ISA::BEQ.build(Operands {
                rs1: 1,
                rs2: 2,
                imm: 0x10,
                ..Default::default()
            }), // BEQ (branch if x1 == x2)
            ISA::BEQ.build(Operands {
                rs1: 0,
                rs2: 2,
                imm: 0x10,
                ..Default::default()
            }), // BEQ (branch if x0 == x2)
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);
    // ADDI ( x1 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);

    // Check whether the branch occurs (branch to PC + 0x4 if x1 == x2)
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x4);

    // Check whether the branch occurs (branch to PC + 0x10 if x0 == x2)
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x10);
}

#[test]
fn test_BNE() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            ISA::ADDI.build(Operands {
                rd: 1,
                rs1: 0,
                imm: 1,
                ..Default::default()
            }), // ADDI ( x1 := x0 + 1)
            ISA::BNE.build(Operands {
                rs1: 0,
                rs2: 2,
                imm: 0x10,
                ..Default::default()
            }), // BNE (branch if x0 != x2)
            ISA::BNE.build(Operands {
                rs1: 1,
                rs2: 2,
                imm: 0x10,
                ..Default::default()
            }), // BNE (branch if x1 != x2)
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);
    // ADDI ( x1 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);

    // Check that branch did not occur because x0 == x2
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x4);

    // Check whether the branch occurs (branch to PC + 0x10 because x1 != x2)
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x10);
}

#[test]
fn test_BLT() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            ISA::ADDI.build(Operands {
                rd: 1,
                rs1: 0,
                imm: 1,
                ..Default::default()
            }), // ADDI ( x1 := x0 + 1)
            ISA::BLT.build(Operands {
                rs1: 0,
                rs2: 2,
                imm: 0x10,
                ..Default::default()
            }), // BLT (branch if x0 < x2)
            ISA::BLT.build(Operands {
                rs1: 2,
                rs2: 1,
                imm: 0x10,
                ..Default::default()
            }), // BLT (branch if x2 < x1)
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);
    // ADDI ( x1 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);

    // Check that branch did not occur because x0 >= x2
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x4);

    // Check whether the branch occurs (branch to PC + 0x10 because x2 < x1)
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x10);
}

#[test]
fn test_BGE() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            ISA::ADDI.build(Operands {
                rd: 1,
                rs1: 0,
                imm: 1,
                ..Default::default()
            }), // ADDI ( x1 := x0 + 1)
            ISA::BGE.build(Operands {
                rs1: 2,
                rs2: 1,
                imm: 0x10,
                ..Default::default()
            }), // BGE (branch if x2 >= x1)
            ISA::BGE.build(Operands {
                rs1: 0,
                rs2: 2,
                imm: 0x10,
                ..Default::default()
            }), // BGE (branch if x0 >= x2)
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);
    // ADDI ( x1 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);

    // Check that branch did not occur because x2 < x1
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x4);

    // Check whether the branch occurs (branch to PC + 0x10 because x0 >= x2)
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x10);
}

#[test]
fn test_BLTU() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            ISA::ADDI.build(Operands {
                rd: 1,
                rs1: 0,
                imm: 1,
                ..Default::default()
            }), // ADDI ( x1 := x0 + 1)
            ISA::BLTU.build(Operands {
                rs1: 0,
                rs2: 2,
                imm: 0x10,
                ..Default::default()
            }), // BLTU (branch if x0 < x2)
            ISA::BLTU.build(Operands {
                rs1: 2,
                rs2: 1,
                imm: 0x10,
                ..Default::default()
            }), // BLTU (branch if x2 < x1)
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);
    // ADDI ( x1 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);

    // Check that branch did not occur because x0 >= x2
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x4);

    // Check whether the branch occurs (branch to PC + 0x10 because x2 < x1)
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x10);
}

#[test]
fn test_BGEU() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            ISA::ADDI.build(Operands {
                rd: 1,
                rs1: 0,
                imm: 1,
                ..Default::default()
            }), // ADDI ( x1 := x0 + 1)
            ISA::BGEU.build(Operands {
                rs1: 2,
                rs2: 1,
                imm: 0x10,
                ..Default::default()
            }), // BGEU (branch if x2 >= x1)
            ISA::BGEU.build(Operands {
                rs1: 0,
                rs2: 2,
                imm: 0x10,
                ..Default::default()
            }), // BGEU (branch if x0 >= x2)
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);
    // ADDI ( x1 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);

    // Check that branch did not occur because x2 < x1
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x4);

    // Check whether the branch occurs (branch to PC + 0x10 because x0 >= x2)
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc + 0x10);
}

#[test]
fn test_ADDI() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    // ADDI ( x1 := x0 + 1)
    // ADDI ( x1 := x1 + (-1))
    // ADDI ( x0 := x0 + 1 )
    populate(
        &mut instruction_map,
        &[
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
        ],
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);

    // ADDI ( x1 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[1], 1);
    // ADDI ( x1 := x1 + 1)
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[1], 0);
    // ADDI ( x0 := x0 + 1) <= special case should be a noop
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[0], 0);
}

#[test]
fn test_SLTI() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<u32, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<u32, u8> = BTreeMap::new();

    // SLTI ( x1 := x0 + 1)
    // SLTI ( x1 := x1 + (-1))
    // SLTI ( x0 := x0 + 1 )
}
