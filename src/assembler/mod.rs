#[cfg(test)]
mod tests;
mod assembled_program;

use assembled_program::AssemblyErr;
pub use assembled_program::{AssembledProgram, Address};

use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use crate::isa::{Instruction, InstructionDefinition, InstructionFormat, Operands, ISA};

#[derive(Debug)]
struct DataItem {
    size: usize, // in bytes
    values: Vec<u8>,
}

pub fn assemble(program: &str) -> Result<AssembledProgram, AssemblyErr> {
    program.parse::<AssembledProgram>()
}

fn clean_line(line: &str) -> &str {
    // Remove leading/trailing whitespace and comments
    if let Some(hash_pos) = line.find('#') {
        // Remove comments
        line[..hash_pos].trim()
    } else {
        line.trim()
    }
}

fn split_label_and_content(line: &str) -> (Option<&str>, &str) {
    if let Some(colon_pos) = line.find(':') {
        // Label found
        let (raw_label, rest) = line.split_at(colon_pos);
        let content = rest[1..].trim();
        let label = raw_label.trim();
        (Some(label), content)
    } else {
        // No label
        (None, line)
    }
}

fn parse_data_line(line: &str) -> Result<Option<(String, DataItem)>, String> {
    if line.ends_with(':') {
        return Ok(None);
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Invalid data directive".to_string());
    }

    let directive = parts[0];
    let joined = parts[1..].join(" ");
    let values: Vec<&str> = joined.split(',').map(|s| s.trim()).collect();

    match directive {
        ".byte" => {
            let bytes = values
                .iter()
                .map(|v| parse_number(v))
                .collect::<Result<Vec<u8>, _>>()?;
            Ok(Some((
                String::new(),
                DataItem {
                    size: bytes.len(),
                    values: bytes,
                },
            )))
        }
        ".word" => {
            let mut bytes = Vec::new();
            for value in &values {
                let word = parse_number(value)? as u32;
                bytes.extend_from_slice(&word.to_le_bytes());
            }
            Ok(Some((
                String::new(),
                DataItem {
                    size: bytes.len(),
                    values: bytes,
                },
            )))
        }
        ".ascii" | ".string" => {
            let text = values
                .join(",")
                .trim_matches('"')
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r");
            let mut bytes = text.as_bytes().to_vec();
            if directive == ".string" {
                bytes.push(0);
            }
            Ok(Some((
                String::new(),
                DataItem {
                    size: bytes.len(),
                    values: bytes,
                },
            )))
        }
        _ => Err(format!("Unknown data directive: {}", directive)),
    }
}

fn parse_number(value: &str) -> Result<u8, String> {
    let value = value.trim();
    if value.starts_with("0x") {
        u8::from_str_radix(&value[2..], 16)
    } else {
        value.parse::<u8>()
    }
    .map_err(|_| format!("Invalid numeric value: {}", value))
}

fn parse_instruction(
    line: &str,
    text_labels: &HashMap<String, u32>,
    data_labels: &HashMap<String, u32>,
    current_address: u32,
) -> Result<Instruction, String> {
    let parts: Vec<&str> = line
        .split(|c| c == ' ' || c == ',')
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        return Err("Empty instruction".to_string());
    }

    let name = parts[0].to_uppercase();
    let def = ISA::from_str(&name)
        .map_err(|_| format!("Unknown instruction: {}", name))?
        .definition();

    if def.format == InstructionFormat::I && def.opcode == 0b0000011
        || def.format == InstructionFormat::S && def.opcode == 0b0100011
    {
        if parts.len() == 3 && data_labels.contains_key(parts[2]) {
            let offset = data_labels[parts[2]];
            let modified_addr = format!("{}(x0)", offset);
            let mut modified_parts = parts.to_vec();
            modified_parts[2] = &modified_addr;
            return match def.format {
                InstructionFormat::I => parse_i_type(&modified_parts, def.clone()),
                InstructionFormat::S => parse_s_type(&modified_parts, def.clone()),
                _ => unreachable!(),
            };
        }
    }

    match def.format {
        InstructionFormat::R => parse_r_type(&parts, def),
        InstructionFormat::I => parse_i_type(&parts, def),
        InstructionFormat::S => parse_s_type(&parts, def),
        InstructionFormat::B => parse_b_type(&parts, def, text_labels, current_address),
        InstructionFormat::U => parse_u_type(&parts, def),
        InstructionFormat::J => parse_j_type(&parts, def, text_labels, current_address),
    }
}

fn parse_r_type(parts: &[&str], def: InstructionDefinition) -> Result<Instruction, String> {
    if parts.len() != 4 {
        return Err("R-type instructions need 3 registers".to_string());
    }
    let operands = Operands {
        rd: parse_register(parts[1])?,
        rs1: parse_register(parts[2])?,
        rs2: parse_register(parts[3])?,
        imm: 0,
    };
    Ok(Instruction::from_def_operands(def, operands))
}

fn parse_i_type(parts: &[&str], def: InstructionDefinition) -> Result<Instruction, String> {
    match def.opcode {
        0b0000011 => parse_load_type(&parts, def),
        0b1110011 => {
            // Special handling for ECALL/EBREAK
            if parts.len() != 1 {
                return Err("ECALL/EBREAK instructions take no operands".to_string());
            }

            let operands = Operands {
                rd: 0,
                rs1: 0,
                imm: match parts[0] {
                    "ECALL" => 0,
                    "EBREAK" => 1,
                    _ => unreachable!(),
                },
                ..Default::default()
            };
            Ok(Instruction::from_def_operands(def, operands))
        }
        0b0001111 => {
            if parts.len() != 1 {
                return Err("FENCE instruction takes no operands".to_string());
            }

            let operands = Operands {
                rd: 0,
                rs1: 0,
                imm: 0,
                ..Default::default()
            };
            Ok(Instruction::from_def_operands(def, operands))
        }
        _ => {
            if parts.len() != 4 {
                return Err("I-type instructions need 2 registers and an immediate".to_string());
            }

            let mut imm = parse_immediate(parts[3])?;
            if imm > 2047 || imm < -2048 {
                return Err("Immediate value out of range (-2048 to 2047)".to_string());
            }

            if let Some(funct7) = def.funct7 {
                // Shift instructions (immediate split into funct7 and shamt)
                if def.opcode == 0b0010011 && (def.funct3 == Some(0x1) || def.funct3 == Some(0x5)) {
                    // SLLI, SRLI, SRAI
                    let shamt = imm & 0x1F; // Bottom 5 bits only
                    imm = ((funct7 as i32) << 5) | shamt; // Combine funct7 and shamt
                }
            }

            let operands = Operands {
                rd: parse_register(parts[1])?,
                rs1: parse_register(parts[2])?,
                imm,
                ..Default::default()
            };
            Ok(Instruction::from_def_operands(def, operands))
        }
    }
}

fn parse_load_type(parts: &[&str], def: InstructionDefinition) -> Result<Instruction, String> {
    if parts.len() != 3 {
        return Err("Load instructions need a register and a memory address".to_string());
    }

    let (offset, base) = parse_mem_address(parts[2])?;

    let operands = Operands {
        rd: parse_register(parts[1])?,
        rs1: base,
        imm: offset,
        ..Default::default()
    };
    Ok(Instruction::from_def_operands(def, operands))
}

fn parse_s_type(parts: &[&str], def: InstructionDefinition) -> Result<Instruction, String> {
    if parts.len() != 3 {
        return Err("Store instructions need a register and a memory address".to_string());
    }

    let (offset, base) = parse_mem_address(parts[2])?;

    let operands = Operands {
        rs1: base,
        rs2: parse_register(parts[1])?,
        imm: offset,
        ..Default::default()
    };
    Ok(Instruction::from_def_operands(def, operands))
}

fn parse_b_type(
    parts: &[&str],
    def: InstructionDefinition,
    labels: &HashMap<String, u32>,
    current_address: u32,
) -> Result<Instruction, String> {
    if parts.len() != 4 {
        return Err("B-type instructions need 2 registers and a label".to_string());
    }

    let target = labels
        .get(parts[3])
        .ok_or(format!("Undefined label: {}", parts[3]))?;

    let offset = (*target as i32) - (current_address as i32);
    if offset & 1 != 0 {
        return Err("Branch target must be 2-byte aligned".to_string());
    }
    if offset > 4095 || offset < -4096 {
        return Err("Branch offset out of range (-4096 to +4095)".to_string());
    }

    let operands = Operands {
        rs1: parse_register(parts[1])?,
        rs2: parse_register(parts[2])?,
        imm: offset,
        ..Default::default()
    };
    Ok(Instruction::from_def_operands(def, operands))
}

fn parse_u_type(parts: &[&str], def: InstructionDefinition) -> Result<Instruction, String> {
    if parts.len() != 3 {
        return Err("U-type instructions need a register and an immediate".to_string());
    }

    let imm = parse_immediate(parts[2])?;
    let imm_value = ((imm as u32) & 0xFFFFF) << 12;

    let operands = Operands {
        rd: parse_register(parts[1])?,
        imm: imm_value as i32,
        ..Default::default()
    };
    Ok(Instruction::from_def_operands(def, operands))
}

fn parse_j_type(
    parts: &[&str],
    def: InstructionDefinition,
    labels: &HashMap<String, u32>,
    current_address: u32,
) -> Result<Instruction, String> {
    if parts.len() != 3 {
        return Err("J-type instructions need a register and a label/offset".to_string());
    }

    let offset = if let Ok(imm) = parse_immediate(parts[2]) {
        if imm & 1 != 0 {
            return Err("Jump target must be 2-byte aligned".to_string());
        }
        imm
    } else {
        let target = labels
            .get(parts[2])
            .ok_or(format!("Undefined label: {}", parts[2]))?;
        let offset = (*target as i32) - (current_address as i32);
        if offset & 1 != 0 {
            return Err("Jump target must be 2-byte aligned".to_string());
        }
        if offset > 1048575 || offset < -1048576 {
            return Err("Jump offset out of range (-1048576 to +1048575)".to_string());
        }
        offset
    };

    let operands = Operands {
        rd: parse_register(parts[1])?,
        imm: offset,
        ..Default::default()
    };
    Ok(Instruction::from_def_operands(def, operands))
}

fn parse_mem_address(addr: &str) -> Result<(i32, u32), String> {
    let parts: Vec<&str> = addr
        .split(|c| c == '(' || c == ')')
        .filter(|s| !s.is_empty())
        .collect();

    if parts.len() != 2 {
        return Err("Memory address must be in format: offset(register)".to_string());
    }

    let offset = parse_immediate(parts[0])?;
    if offset > 2047 || offset < -2048 {
        return Err("Memory offset out of range (-2048 to 2047)".to_string());
    }

    let reg = parse_register(parts[1])?;

    Ok((offset, reg))
}

fn parse_register(reg: &str) -> Result<u32, String> {
    let reg = reg.trim().to_lowercase();
    if !reg.starts_with('x') {
        return Err(format!("Invalid register: {}", reg));
    }

    match reg[1..].parse::<u32>() {
        Ok(num) if num < 32 => Ok(num),
        _ => Err(format!("Invalid register number: {}", reg)),
    }
}

fn parse_immediate(value: &str) -> Result<i32, String> {
    let value = value.trim();
    let (is_negative, value) = if value.starts_with('-') {
        (true, &value[1..])
    } else {
        (false, value)
    };

    let abs_value = if value.starts_with("0x") {
        i32::from_str_radix(&value[2..], 16)
    } else {
        value.parse::<i32>()
    }
    .map_err(|_| format!("Invalid immediate value: {}", value))?;

    if is_negative {
        Ok(-abs_value)
    } else {
        Ok(abs_value)
    }
}
