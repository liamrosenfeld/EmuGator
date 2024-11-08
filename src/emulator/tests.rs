use std::io::Read;

use dioxus::html::data;
use dioxus_logger::tracing::level_filters::STATIC_MAX_LEVEL;

use super::*;

fn write(map: &mut BTreeMap<XLEN, u8>, address: XLEN, bytes: &[u8]) {
    for (i, &byte) in bytes.iter().enumerate() {
        map.insert(address + i as XLEN, byte);
    }
}

fn populate(map: &mut BTreeMap<XLEN, u8>, instructions: &[Instruction]) {
    for (i, &instruction) in instructions.iter().enumerate() {
        write(map, (4 * i) as XLEN, &instruction.instr.to_le_bytes());
    }
}

#[test]
fn test_LUI() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    // LUI ( x1 := 0x12345000)
    populate(
        &mut instruction_map,
        &[
            Instruction::U(0b0110111, 1, 0x12345000),
            Instruction::U(0b0110111, 0, 0x12345000)
        ]
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

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    // AUIPC ( x1 := PC + 0x12345000)
    populate(
        &mut instruction_map,
        &[
            Instruction::U(0b0010111, 1, 0x12345000)
        ]
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);

    // After AUIPC, x1 should hold the value (PC + 0x12345000)
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[1], emulator.state().pipeline.datapath.instr_addr_o + 0x12345000);
}

#[test]
fn test_JAL() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    // JAL ( x1 := PC + 4, jump to PC + 0x100)
    populate(
        &mut instruction_map,
        &[Instruction::J(0b1101111, 1, 0x100)]
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
fn test_JAL_neg_offset() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    // JAL ( x1 := PC + 4, jump to PC + 0xFFC)
    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b0010011, 5, 0b000, 0, 1),  // ADDI ( x5 := x0 + 1)
            Instruction::I(0b0010011, 5, 0b000, 0, 1),  // ADDI ( x5 := x0 + 1)
            Instruction::J(0b1101111, 1, -4)         // JAL (pc = pc - 4)
        ]        
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);
    // ADDI ( x5 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);
    // ADDI ( x5 := x0 + 1)
    emulator.clock(&instruction_map, &mut data_map);

    // After JAL, x1 should contain PC + 4, and the PC should jump to PC + 0x840
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[1], pc + 4);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, pc - 0x04);
}

#[test]
#[should_panic(expected = "JAL instruction immediate it not on a 4-byte boundary")]
fn test_JAL_panic() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    // JAL ( x1 := PC + 4, jump to PC + 0x123)
    populate(
        &mut instruction_map,
        &[Instruction::J(0b1101111, 1, 0x123)]
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);

    // After JAL, x1 should contain PC + 4, and the PC should jump to PC + 0x100
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
}

#[test]
fn test_JALR() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b1100111, 1, 0b000, 2, 0x4) // JALR ( x1 := PC + 4, jump to (x2 + 0x4) & ~1)
        ] 
    );

    // Instruction fetch
    emulator.clock(&instruction_map, &mut data_map);

    // After JALR, x1 should contain PC + 4, and the PC should jump to (x2 + 0x4) & ~1
    let pc = emulator.state().pipeline.datapath.instr_addr_o;
    emulator.clock(&instruction_map, &mut data_map);
    assert_eq!(emulator.state().x[1], pc + 4);
    assert_eq!(emulator.state().pipeline.datapath.instr_addr_o, (pc + emulator.state().x[2] + 0x4) & !1);
}

#[test]
fn test_BEQ() {
    let mut emulator: Emulator = Emulator::new();

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b0010011, 1, 0b000, 0, 1),      // ADDI ( x1 := x0 + 1)
            Instruction::B(0b1100011, 0b000, 1, 2, 0x10),   // BEQ (branch if x1 == x2)
            Instruction::B(0b1100011, 0b000, 0, 2, 0x10)    // BEQ (branch if x0 == x2)
        ]
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

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b0010011, 1, 0b000, 0, 1),  // ADDI ( x1 := x0 + 1)
            Instruction::B(0b1100011, 0b001, 0, 2, 0x10),   // BNE (branch if x0 != x2)
            Instruction::B(0b1100011, 0b001, 1, 2, 0x10),   // BNE (branch if x1 != x2)
        ]
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

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b0010011, 1, 0b000, 0, 1),  // ADDI ( x1 := x0 + 1)
            Instruction::B(0b1100011, 0b100, 0, 2, 0x10),   // BLT (branch if x0 < x2)
            Instruction::B(0b1100011, 0b100, 2, 1, 0x10),   // BLT (branch if x2 < x1)
        ]
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

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b0010011, 1, 0b000, 0, 1),  // ADDI ( x1 := x0 + 1)
            Instruction::B(0b1100011, 0b101, 2, 1, 0x10),   // BGE (branch if x2 >= x1)
            Instruction::B(0b1100011, 0b101, 0, 2, 0x10),   // BGE (branch if x0 >= x2)
        ]
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

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b0010011, 1, 0b000, 0, 1),  // ADDI ( x1 := x0 + 1)
            Instruction::B(0b1100011, 0b110, 0, 2, 0x10),   // BLTU (branch if x0 < x2)
            Instruction::B(0b1100011, 0b110, 2, 1, 0x10),   // BLTU (branch if x2 < x1)
        ]
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

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b0010011, 1, 0b000, 0, 1),  // ADDI ( x1 := x0 + 1)
            Instruction::B(0b1100011, 0b111, 2, 1, 0x10),   // BGEU (branch if x2 >= x1)
            Instruction::B(0b1100011, 0b111, 0, 2, 0x10),   // BGEU (branch if x0 >= x2)
        ]
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

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    // ADDI ( x1 := x0 + 1)
    // ADDI ( x1 := x1 + (-1))
    // ADDI ( x0 := x0 + 1 )
    populate(
        &mut instruction_map,
        &[
            Instruction::I(0b0010011, 1, 0b000, 0, 1),
            Instruction::I(0b0010011, 1, 0b000, 1, -1),
            Instruction::I(0b0010011, 0, 0b000, 0, 1)
        ]
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

    let mut instruction_map: BTreeMap<XLEN, u8> = BTreeMap::new();
    let mut data_map: BTreeMap<XLEN, u8> = BTreeMap::new();

    // SLTI ( x1 := x0 + 1)
    // SLTI ( x1 := x1 + (-1))
    // SLTI ( x0 := x0 + 1 )

}


#[test]
fn test_bits() {
    let ten = 0b1010;

    assert_eq!(bits!(ten, 0), 0b0);
    assert_eq!(bits!(ten, 1), 0b1);
    assert_eq!(bits!(ten, 2), 0b0);
    assert_eq!(bits!(ten, 3), 0b1);
    
    assert_eq!(bits!(ten, 0, 2), 0b10);
    assert_eq!(bits!(ten, 1, 3), 0b101);
    assert_eq!(bits!(ten, 3;1), 0b101);

}

#[test]
fn test_bitmask() {
    assert_eq!(bitmask!(0, 5), 0b11111);
    assert_eq!(bitmask!(10;5), 0b11111100000);
    assert_eq!(bitmask!(5), 0b100000);
}