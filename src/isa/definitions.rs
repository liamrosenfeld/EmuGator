use strum::EnumString;

use super::Instruction;

#[derive(Default, Debug)]
pub struct Operands {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionDefinition {
    pub name: &'static str,
    pub opcode: u8,
    pub funct3: Option<u8>,
    pub funct7: Option<u8>,
    pub format: InstructionFormat,
}

impl InstructionDefinition {
    pub fn from_instr(instr: Instruction) -> Option<InstructionDefinition> {
        let opcode = instr.opcode();
        let funct3 = instr.funct3();
        let funct7 = instr.funct7();

        ISA::from_opcode(opcode, Some(funct3), Some(funct7)).map(|isa| isa.definition())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionFormat {
    R,
    I,
    S,
    B,
    U,
    J,
}

#[derive(EnumString, Debug)]
pub enum ISA {
    ADD,
    SUB,
    SLT,
    SLTU,
    AND,
    OR,
    XOR,
    SLL,
    SRL,
    SRA,
    ADDI,
    SLTI,
    SLTIU,
    ANDI,
    ORI,
    XORI,
    SLLI,
    SRLI,
    SRAI,
    JALR,
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,
    LW,
    LH,
    LHU,
    LB,
    LBU,
    FENCE,
    ECALL,
    EBREAK,
    SW,
    SH,
    SB,
    BEQ,
    BNE,
    BLT,
    BLTU,
    BGE,
    BGEU,
    LUI,
    AUIPC,
    JAL,
}

impl ISA {
    pub fn build(&self, operands: Operands) -> Instruction {
        Instruction::from_def_operands(self.definition(), operands)
    }

    pub fn definition(&self) -> InstructionDefinition {
        use ISA::*;
        match self {
            ADD => InstructionDefinition {
                name: "ADD",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x0),
                funct7: Some(0x00),
            },
            SUB => InstructionDefinition {
                name: "SUB",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x0),
                funct7: Some(0x20),
            },
            SLT => InstructionDefinition {
                name: "SLT",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x2),
                funct7: Some(0x00),
            },
            SLTU => InstructionDefinition {
                name: "SLTU",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x3),
                funct7: Some(0x00),
            },
            AND => InstructionDefinition {
                name: "AND",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x7),
                funct7: Some(0x00),
            },
            OR => InstructionDefinition {
                name: "OR",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x6),
                funct7: Some(0x00),
            },
            XOR => InstructionDefinition {
                name: "XOR",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x4),
                funct7: Some(0x00),
            },
            SLL => InstructionDefinition {
                name: "SLL",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x1),
                funct7: Some(0x00),
            },
            SRL => InstructionDefinition {
                name: "SRL",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x5),
                funct7: Some(0x00),
            },
            SRA => InstructionDefinition {
                name: "SRA",
                format: InstructionFormat::R,
                opcode: 0b0110011,
                funct3: Some(0x5),
                funct7: Some(0x20),
            },

            // I-type instructions
            ADDI => InstructionDefinition {
                name: "ADDI",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x0),
                funct7: None,
            },
            SLTI => InstructionDefinition {
                name: "SLTI",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x2),
                funct7: None,
            },
            SLTIU => InstructionDefinition {
                name: "SLTIU",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x3),
                funct7: None,
            },
            ANDI => InstructionDefinition {
                name: "ANDI",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x7),
                funct7: None,
            },
            ORI => InstructionDefinition {
                name: "ORI",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x6),
                funct7: None,
            },
            XORI => InstructionDefinition {
                name: "XORI",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x4),
                funct7: None,
            },
            SLLI => InstructionDefinition {
                name: "SLLI",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x1),
                funct7: Some(0x00),
            },
            SRLI => InstructionDefinition {
                name: "SRLI",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x5),
                funct7: Some(0x00),
            },
            SRAI => InstructionDefinition {
                name: "SRAI",
                format: InstructionFormat::I,
                opcode: 0b0010011,
                funct3: Some(0x5),
                funct7: Some(0x20),
            },
            JALR => InstructionDefinition {
                name: "JALR",
                format: InstructionFormat::I,
                opcode: 0b1100111,
                funct3: Some(0x0),
                funct7: None,
            },
            CSRRW => InstructionDefinition {
                name: "CSRRW",
                format: InstructionFormat::I,
                opcode: 0b1110011,
                funct3: Some(0x1),
                funct7: Some(0x00),
            },
            CSRRS => InstructionDefinition {
                name: "CSRRS",
                format: InstructionFormat::I,
                opcode: 0b1110011,
                funct3: Some(0x2),
                funct7: Some(0x0),
            },
            CSRRC => InstructionDefinition {
                name: "CSRRC",
                format: InstructionFormat::I,
                opcode: 0b1110011,
                funct3: Some(0x3),
                funct7: Some(0x0),
            },
            CSRRWI => InstructionDefinition {
                name: "CSRRWI",
                format: InstructionFormat::I,
                opcode: 0b1110011,
                funct3: Some(0x5),
                funct7: Some(0x6),
            },
            CSRRSI => InstructionDefinition {
                name: "CSRRSI",
                format: InstructionFormat::I,
                opcode: 0b1110011,
                funct3: Some(0x6),
                funct7: Some(0x2),
            },
            CSRRCI => InstructionDefinition {
                name: "CSRRCI",
                format: InstructionFormat::I,
                opcode: 0b1110011,
                funct3: Some(0x7),
                funct7: Some(0x2),
            },

            // Load instructions (I-type)
            LW => InstructionDefinition {
                name: "LW",
                format: InstructionFormat::I,
                opcode: 0b0000011,
                funct3: Some(0x2),
                funct7: None,
            },
            LH => InstructionDefinition {
                name: "LH",
                format: InstructionFormat::I,
                opcode: 0b0000011,
                funct3: Some(0x1),
                funct7: None,
            },
            LHU => InstructionDefinition {
                name: "LHU",
                format: InstructionFormat::I,
                opcode: 0b0000011,
                funct3: Some(0x5),
                funct7: None,
            },
            LB => InstructionDefinition {
                name: "LB",
                format: InstructionFormat::I,
                opcode: 0b0000011,
                funct3: Some(0x0),
                funct7: None,
            },
            LBU => InstructionDefinition {
                name: "LBU",
                format: InstructionFormat::I,
                opcode: 0b0000011,
                funct3: Some(0x4),
                funct7: None,
            },

            // Special I-type instructions
            FENCE => InstructionDefinition {
                name: "FENCE",
                format: InstructionFormat::I,
                opcode: 0b0001111,
                funct3: Some(0x0),
                funct7: None,
            },
            ECALL => InstructionDefinition {
                name: "ECALL",
                format: InstructionFormat::I,
                opcode: 0b1110011,
                funct3: Some(0x0),
                funct7: None,
            },
            EBREAK => InstructionDefinition {
                name: "EBREAK",
                format: InstructionFormat::I,
                opcode: 0b1110011,
                funct3: Some(0x0),
                funct7: None,
            },

            // S-type instructions
            SW => InstructionDefinition {
                name: "SW",
                format: InstructionFormat::S,
                opcode: 0b0100011,
                funct3: Some(0x2),
                funct7: None,
            },
            SH => InstructionDefinition {
                name: "SH",
                format: InstructionFormat::S,
                opcode: 0b0100011,
                funct3: Some(0x1),
                funct7: None,
            },
            SB => InstructionDefinition {
                name: "SB",
                format: InstructionFormat::S,
                opcode: 0b0100011,
                funct3: Some(0x0),
                funct7: None,
            },

            // B-type instructions
            BEQ => InstructionDefinition {
                name: "BEQ",
                format: InstructionFormat::B,
                opcode: 0b1100011,
                funct3: Some(0x0),
                funct7: None,
            },
            BNE => InstructionDefinition {
                name: "BNE",
                format: InstructionFormat::B,
                opcode: 0b1100011,
                funct3: Some(0x1),
                funct7: None,
            },
            BLT => InstructionDefinition {
                name: "BLT",
                format: InstructionFormat::B,
                opcode: 0b1100011,
                funct3: Some(0x4),
                funct7: None,
            },
            BLTU => InstructionDefinition {
                name: "BLTU",
                format: InstructionFormat::B,
                opcode: 0b1100011,
                funct3: Some(0x6),
                funct7: None,
            },
            BGE => InstructionDefinition {
                name: "BGE",
                format: InstructionFormat::B,
                opcode: 0b1100011,
                funct3: Some(0x5),
                funct7: None,
            },
            BGEU => InstructionDefinition {
                name: "BGEU",
                format: InstructionFormat::B,
                opcode: 0b1100011,
                funct3: Some(0x7),
                funct7: None,
            },

            // U-type instructions
            LUI => InstructionDefinition {
                name: "LUI",
                format: InstructionFormat::U,
                opcode: 0b0110111,
                funct3: None,
                funct7: None,
            },
            AUIPC => InstructionDefinition {
                name: "AUIPC",
                format: InstructionFormat::U,
                opcode: 0b0010111,
                funct3: None,
                funct7: None,
            },

            // J-type instructions
            JAL => InstructionDefinition {
                name: "JAL",
                format: InstructionFormat::J,
                opcode: 0b1101111,
                funct3: None,
                funct7: None,
            },
        }
    }

    pub fn from_opcode(opcode: u8, funct3: Option<u8>, funct7: Option<u8>) -> Option<ISA> {
        todo!()
    }
}
