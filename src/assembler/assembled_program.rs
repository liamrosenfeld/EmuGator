use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

#[derive(Debug)]
pub struct AssembledProgram {
    pub instruction_memory: BTreeMap<u32, u8>,
    pub data_memory: BTreeMap<u32, u8>,
    pub source_map: BTreeMap<u32, usize>,
    pub labels: HashMap<String, u32>,
    pub data_labels: HashMap<String, u32>,
}

impl AssembledProgram {
    pub fn new() -> Self {
        AssembledProgram {
            instruction_memory: BTreeMap::new(),
            data_memory: BTreeMap::new(),
            source_map: BTreeMap::new(),
            labels: HashMap::new(),
            data_labels: HashMap::new(),
        }
    }

    pub fn get_section_start(&self, section: Section) -> u32 {
        match section {
            Section::Text => self.source_map.keys().next().copied().unwrap_or(0),
            Section::Data => self.data_memory.keys().next().copied().unwrap_or(0),
        }
    }

    pub fn add_label(&mut self, label: String, address: u32, is_data: bool) {
        if is_data {
            self.data_labels.insert(label, address);
        } else {
            self.labels.insert(label, address);
        }
    }

    pub fn add_instruction(&mut self, address: u32, encoded: u32, line_num: usize) {
        self.instruction_memory
            .insert(address, (encoded & 0xFF) as u8);
        self.instruction_memory
            .insert(address + 1, ((encoded >> 8) & 0xFF) as u8);
        self.instruction_memory
            .insert(address + 2, ((encoded >> 16) & 0xFF) as u8);
        self.instruction_memory
            .insert(address + 3, ((encoded >> 24) & 0xFF) as u8);

        self.source_map.insert(address, line_num);
    }

    pub fn add_data(&mut self, address: u32, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.data_memory.insert(address + i as u32, byte);
        }
    }

    pub fn emulator_maps(
        &self,
    ) -> (
        &BTreeMap<u32, u8>,
        &BTreeMap<u32, usize>,
        &BTreeMap<u32, u8>,
    ) {
        (
            &self.instruction_memory,
            &self.source_map,
            &self.data_memory,
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Section {
    Data,
    Text,
}
