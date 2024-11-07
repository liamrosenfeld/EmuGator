use crate::{bitmask, bits};
use strum::EnumString;

#[derive(Default, Debug)]
pub struct Operands {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub imm: i32,
}

#[derive(Debug)]
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

        ISA::from_opcode(opcode, Some(funct3), Some(funct7)).map(|isa| isa.get_instruction())
    }

    pub fn build(&self, Operands { rd, rs1, rs2, imm }: Operands) -> Instruction {
        Instruction::new(
            self.format,
            self.opcode as u32,
            rd,
            self.funct3.unwrap_or_default() as u32,
            rs1,
            rs2,
            self.funct7.unwrap_or_default() as u32,
            imm,
        )
    }
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
    pub fn build(&self, fields: Operands) -> Instruction {
        self.get_instruction().build(fields)
    }

    fn get_instruction(&self) -> InstructionDefinition {
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

    fn from_str(name: &str) -> Option<ISA> {
        match name {
            "ADD" => Some(ISA::ADD),
            "ADDI" => Some(ISA::ADDI),
            "JAL" => Some(ISA::JAL),
            _ => None,
        }
    }

    fn from_opcode(opcode: u8, funct3: Option<u8>, funct7: Option<u8>) -> Option<ISA> {
        match (opcode, funct3, funct7) {
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InstructionFormat {
    R,
    I,
    S,
    B,
    U,
    J,
    CUSTOM,
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub(crate) instr: u32,
}

impl Instruction {
    pub fn new(
        format: InstructionFormat,
        opcode: u32,
        rd: u32,
        funct3: u32,
        rs1: u32,
        rs2: u32,
        funct7: u32,
        imm: i32,
    ) -> Instruction {
        assert_eq!(opcode, bits!(opcode,6;0));
        assert_eq!(rd, bits!(rd,4;0));
        assert_eq!(funct3, bits!(funct3,2;0));
        assert_eq!(rs1, bits!(rs1,4;0));
        assert_eq!(rs2, bits!(rs2,4;0));
        assert_eq!(funct7, bits!(funct7,6;0));
        match format {
            InstructionFormat::R => Self::R(opcode, rd, funct3, rs1, rs2, funct7),
            InstructionFormat::I => Self::I(opcode, rd, funct3, rs1, imm),
            InstructionFormat::S => Self::S(opcode, funct3, rs1, rs2, imm),
            InstructionFormat::B => Self::B(opcode, funct3, rs1, rs2, imm),
            InstructionFormat::U => Self::U(opcode, rd, imm),
            InstructionFormat::J => Self::J(opcode, rd, imm),
            _ => panic!("Invalid instruction format"),
        }
    }

    #[allow(non_snake_case)]
    fn R(opcode: u32, rd: u32, funct3: u32, rs1: u32, rs2: u32, funct7: u32) -> Instruction {
        Self {
            instr: (funct7 << 25 | rs2 << 20 | rs1 << 15 | funct3 << 12 | rd << 7 | opcode << 0),
        }
    }

    #[allow(non_snake_case)]
    fn I(opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: i32) -> Instruction {
        assert!((imm == bits!(imm,11;0)) || (imm & bitmask!(31;11) == bitmask!(31;11)));
        let imm: u32 = imm as u32;
        Self {
            instr: (imm << 20 | rs1 << 15 | funct3 << 12 | rd << 7 | opcode << 0),
        }
    }

    #[allow(non_snake_case)]
    fn S(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> Instruction {
        assert!((imm == bits!(imm,11;0)) || (imm & bitmask!(31;11) == bitmask!(31;11)));
        let imm: u32 = imm as u32;
        Self {
            instr: (bits!(imm,11;5) << 25
                | rs2 << 20
                | rs1 << 15
                | funct3 << 12
                | bits!(imm,4;0) << 7
                | opcode << 0),
        }
    }

    #[allow(non_snake_case)]
    fn B(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> Instruction {
        assert!((imm == bits!(imm,11;0)) || (imm & bitmask!(31;11) == bitmask!(31;11)));
        let imm: u32 = imm as u32;
        Self {
            instr: (bits!(imm, 12) << 31
                | bits!(imm,10;5) << 25
                | rs2 << 20
                | rs1 << 15
                | funct3 << 12
                | bits!(imm,4;1) << 8
                | bits!(imm, 11) << 7
                | opcode << 0),
        }
    }

    #[allow(non_snake_case)]
    fn U(opcode: u32, rd: u32, imm: i32) -> Instruction {
        assert_eq!(imm, bits!(imm,31;12) << 12);
        let imm: u32 = imm as u32;
        Self {
            instr: (bits!(imm,31;12) << 12 | rd << 7 | opcode << 0),
        }
    }

    #[allow(non_snake_case)]
    fn J(opcode: u32, rd: u32, imm: i32) -> Instruction {
        if imm >= 0 {
            assert_eq!(imm, bits!(imm,20;1) << 1);
        } else {
            assert_eq!(bitmask!(12), bits!(imm, 20, 12));
        }
        let imm: u32 = imm as u32;
        Self {
            instr: (bits!(imm, 20) << 31
                | bits!(imm,10;1) << 21
                | bits!(imm, 11) << 20
                | bits!(imm,19;12) << 12
                | rd << 7
                | opcode << 0),
        }
    }

    pub fn opcode(&self) -> u8 {
        bits!(self.instr,6;0) as u8
    }

    pub fn immediate(&self, format: InstructionFormat) -> Result<i32, ()> {
        match format {
            InstructionFormat::I => {
                Ok((bits!(self.instr, 31) * bitmask!(31; 11) | bits!(self.instr,30;20)) as i32)
            }
            InstructionFormat::S => Ok((bits!(self.instr, 31) * bitmask!(31; 11)
                | bits!(self.instr,30;25) << 5
                | bits!(self.instr,11;7 )) as i32),
            InstructionFormat::B => Ok((bits!(self.instr, 31) * bitmask!(31; 12)
                | bits!(self.instr, 7) << 11
                | bits!(self.instr,30;25) << 5
                | bits!(self.instr,11;8 ) << 1) as i32),
            InstructionFormat::U => Ok((bits!(self.instr,31;12) << 12) as i32),
            InstructionFormat::J => Ok((bits!(self.instr, 31) * bitmask!(31; 20)
                | bits!(self.instr,19;12) << 12
                | bits!(self.instr, 20) << 11
                | bits!(self.instr,30;25) << 5
                | bits!(self.instr,24;21) << 1) as i32),
            _ => Err(()),
        }
    }

    pub fn rd(&self) -> u8 {
        bits!(self.instr, 7, 5) as u8
    }

    pub fn rs1(&self) -> u8 {
        bits!(self.instr, 15, 5) as u8
    }

    pub fn rs2(&self) -> u8 {
        bits!(self.instr, 20, 5) as u8
    }

    pub fn funct3(&self) -> u8 {
        bits!(self.instr, 12, 3) as u8
    }

    pub fn funct7(&self) -> u8 {
        bits!(self.instr, 25, 7) as u8
    }
}
