use crate::bitmask;

/// Struct representing the datapath for the `cve2_top` module.
/// Taken from https://github.com/openhwgroup/cve2/blob/main/rtl/

#[derive(Copy, Clone)]
pub enum PCSelect {
    PcBoot,
    PcJump,
    PcExc,
    PcERet,
    PcDRet,
}

impl Default for PCSelect {
    fn default() -> Self {
        PCSelect::PcBoot
    }
}

#[derive(Copy, Clone)]
pub enum ALUOperation {
    DEFAULT
}

impl Default for ALUOperation {
    fn default() -> Self {
        ALUOperation::DEFAULT
    }
}

pub struct CVE2IFStageInterface<'a> {
    pub boot_addr_i: &'a u32,                   // datapath.boot_addr
    pub req_i: &'a bool,                        // datapath.core.instr_req_int

    pub instr_req_o: &'a mut bool,              // datapath.instr_req_o
    pub instr_addr_o: &'a mut u32,              // datapath.instr_addr_o
    pub instr_gnt_i: &'a bool,                  // datapath.instr_gnt_i
    pub instr_rvalid_i: &'a bool,               // datapath.instr_rvalid_i
    pub instr_rdata_i: &'a u32,                 // datapath.instr_rdata_i
    pub instr_err_i: &'a bool,                  // datapath.instr_err_i

    pub instr_valid_id_o: &'a mut bool,         // datapath.core.instr_valid_id
    pub instr_new_id_o: &'a mut bool,           // datapath.core.instr_new_id
    pub instr_rdata_id_o: &'a mut u32,          // datapath.core.instr_rdata_id
    // pub instr_rdata_c_id_o: &'a mut u16,     // datapath.core.instr_rdata_c_id
    // pub instr_is_compressed_id_o: &'a mut bool,  // datapath.core.instr_is_compressed_id
    // pub instr_perf_count_id_o: &'a mut bool,     // datapath.core.instr_perf_count_id
    pub instr_fetch_err_o: &'a mut bool,        // datapath.core.instr_fetch_err
    // pub instr_fetch_err_plus2_o: &'a mut bool,   // datapath.core.instr_fetch_err_plus2
    // pub illegal_c_insn_id_o: &'a mut bool,       // datapath.core.illegal_c_insn_id

    pub pc_if_o: &'a mut u32,                   // datapath.core.pc_if
    pub pc_id_o: &'a mut u32,                   // datapath.core.pc_id

    pub instr_valid_clear_i: &'a bool,          // datapath.core.instr_valid_clear
    pub pc_set_i: &'a bool,                     // datapath.core.pc_set
    pub pc_mux_i: &'a PCSelect,                 // datapath.core.pc_mux_id

    pub branch_target_ex_i: &'a u32,            // datapath.core.branch_target_ex

    pub id_in_ready_i: &'a bool,                // datapath.core.id_in_ready
    pub if_busy_o: &'a mut bool,                // datapath.core.if_busy
}

const NUM_REQS: usize = 4;
#[derive(Copy, Clone, Default)]
pub struct CVE2IFStage {
    addr: u32,
}

impl CVE2IFStage {
    pub fn new() -> Self {
        CVE2IFStage {..Default::default()}
    }
}

pub struct CVE2IDStageInterface<'a> {
    pub fetch_enable_i: &'a bool,               // TODO: Find where fetch_enable_i is forwarded from
    pub ctrl_busy_o: &'a mut bool,              // datapath.core.ctrl_busy
    pub illegal_insn_o: &'a mut bool,           // datapath.core.illegal_insn_id
    
    pub instr_valid_i: &'a bool,                // datapath.core.instr_valid_id
    pub instr_rdata_i: &'a u32,                 // datapath.core.instr_rdata_id
    // pub instr_rdata_c_i: &'a u16,            // datapath.core.instr_rdata_c_id
    // pub instr_is_compressed_i: &'a bool,     // datapath.core.instr_is_compressed_id

    pub branch_decision_i: &'a bool,            // datapath.core.branch_decision

    pub instr_first_cycle_o: &'a mut bool,      // datapath.core.instr_first_cycle_id
    pub instr_valid_clear_o: &'a mut bool,      // datapath.core.instr_valid_clear
    pub id_in_ready_o: &'a mut bool,            // datapath.core.id_in_ready
    pub instr_req_o: &'a mut bool,              // datapath.core.instr_req_int
    pub pc_set_o: &'a mut bool,                 // datapath.core.pc_set
    pub pc_mux_o: &'a mut PCSelect,             // datapath.core.pc_mux_id

    pub instr_fetch_err_i: &'a bool,            // datapath.core.instr_fetch_err
    // pub instr_fetch_err_plus2_i: &'a bool,   // datapath.core.instr_fetch_err_plus2
    // pub illegal_c_insn_i: &'a bool,          // datapath.core.illegal_c_insn_id

    pub pc_id_i: &'a u32,                       // datapath.core.pc_id

    pub ex_valid_i: &'a bool,                   // datapath.core.ex_valid
    pub lsu_resp_valid_i: &'a bool,             // datapath.core.lsu_resp_valid

    pub alu_operator_ex_o: &'a mut ALUOperation,// datapath.core.alu_operator_ex
    pub alu_operand_a_ex_o: &'a mut u32,        // datapath.core.alu_operand_a_ex
    pub alu_operand_b_ex_o: &'a mut u32,        // datapath.core.alu_operand_b_ex

    pub imd_val_q_h_eq_o: &'a mut [[bool; 2]; 2],   // datapath.core.imd_val_q_h_ex
    pub imd_val_q_eq_o: &'a mut [u32; 2],           // datapath.core.imd_val_q_ex
    pub imd_val_d_h_eq_i: &'a [[bool; 2]; 2],       // datapath.core.imd_val_d_h_ex
    pub imd_val_d_eq_i: &'a [u32; 2],               // datapath.core.imd_val_d_ex
    pub imd_val_we_ex_i: &'a mut [bool; 2],         // datapath.core.imd_val_we_ex

    pub lsu_req_o: &'a mut bool,                // datapath.core.lsu_req
    pub lsu_we_o: &'a mut bool,                 // datapath.core.lsu_we
    pub lsu_type_o: &'a mut [bool; 2],          // datapath.core.lsu_type
    pub lsu_sign_ext_o: &'a mut bool,           // datapath.core.lsu_sign_ext
    pub lsu_wdata_o: &'a mut u32,               // datapath.core.lsu_wdata

    pub lsu_addr_incr_req_i: &'a bool,          // datapath.core.lsu_addr_incr_req
    pub lsu_addr_last_i: &'a u32,               // datapath.core.lsu_addr_last

    pub lsu_load_err_i: &'a bool,               // datapath.core.lsu_load_err
    pub lsu_store_err_i: &'a bool,              // datapath.core.lsu_store_err

    pub result_ex_i: &'a u32,                   // datapath.core.result_ex
    pub rf_raddr_a_o: &'a mut u8,               // datapath.core.rf_raddr_a
    pub rf_rdata_a_i: &'a u32,                  // datapath.core.rf_rdata_a
    pub rf_raddr_b_o: &'a mut u8,               // datapath.core.rf_raddr_b
    pub rf_rdata_b_i: &'a u32,                  // datapath.core.rf_rdata_b
    pub rf_ren_a_o: &'a mut bool,               // datapath.core.rf_ren_a
    pub rf_ren_b_o: &'a mut bool,               // datapath.core.rf_ren_b
    pub rf_waddr_id_o: &'a mut u8,              // datapath.core.rf_waddr_id
    pub if_wdata_id_o: &'a mut u32,             // datapath.core.rf_wdata_id
    pub rf_we_id_o: &'a mut bool,               // datapath.core.rf_we_id

    pub en_wb_o: &'a mut bool,                  // datapath.core.rf_we_wb
    // pub instr_perf_count_id_o: &'a mut bool,
}
#[derive(Copy, Clone, Default)]
pub struct CVE2IDStage {

}

pub struct CVE2ExecuteBlockInterface<'a> {
    pub alu_operator_i: &'a ALUOperation,   // datapath.core.alu_operator_ex
    pub alu_operand_a_i: &'a u32,           // datapath.core.alu_operand_a_ex
    pub alu_operand_b_i: &'a u32,           // datapath.core.alu_operand_b_ex
    pub alu_instr_first_cycle_i: &'a bool,  // datapath.core.instr_first_cycle_id

    pub imd_val_we_o: &'a mut [bool; 2],    // datapath.core.imd_val_we_ex
    pub imd_val_d_h_o: &'a mut [[bool; 2]; 2],  // datapath.core.imd_val_d_h_ex
    pub imd_val_d_o: &'a mut [u32; 2],      // datapath.core.imd_val_d_ex
    pub imd_val_q_h_i: &'a [[bool; 2]; 2],  // datapath.core.imd_val_q_h_ex
    pub imd_val_q_i: &'a [u32; 2],          // datapath.core.imd_val_q_ex

    pub alu_adder_result_ex_o: &'a mut u32, // datapath.core.alu_adder_result_ex
    pub result_ex_o: &'a mut u32,           // datapath.core.result_ex

    pub branch_target_o: &'a mut u32,       // datapath.core.branch_target_ex
    pub branch_decision_o: &'a mut bool,    // datapath.core.branch_decision

    pub ex_valid_o: &'a mut bool,           // datapath.core.ex_valid
}

pub struct CVE2ExecuteBlock {

}

pub struct CVE2LSUInterface<'a> {
    pub data_req_o: &'a mut bool,           // datapath.data_req_o
    pub data_gnt_i: &'a bool,               // datapath.data_gnt_i
    pub data_rvalid_i: &'a bool,            // datapath.data_rvalid_i
    pub data_err_i: &'a bool,               // datapath.data_err_i

    pub data_addr_o: &'a mut u32,           // datapath.data_addr_o
    pub data_we_o: &'a mut bool,            // datapath.data_we_o
    pub data_be_o: &'a mut [bool; 4],       // datapath.data_be_o
    pub data_wdata_o: &'a mut u32,          // datapath.data_wdata_o
    pub data_rdata_i: &'a u32,              // datapath.data_rdata_i

    pub lsu_we_i: &'a bool,                 // datapath.core.lsu_we
    pub lsu_type_i: &'a [bool; 2],          // datapath.core.lsu_type
    pub lsu_wdata_i: &'a u32,               // datapath.core.lsu_wdata
    pub lsu_sign_ext_i: &'a bool,           // datapath.core.lsu_sign_ext

    pub lsu_rdata_o: &'a mut u32,           // datapath.core.rf_wdata_lsu
    pub lsu_rdata_valid_o: &'a mut bool,    // datapath.core.lsu_resp_valid
    pub lsu_req_i: &'a bool,                // datapath.core.lsu_req

    pub adder_result_ex_i: &'a u32,         // datapath.core.alu_adder_result_ex
    
    pub addr_incr_req_o: &'a mut bool,      // datapath.core.lsu_addr_incr_req
    pub addr_last_o: &'a mut u32,           // datapath.core.lsu_addr_last

    pub lsu_resp_valid_o: &'a mut bool,     // datapath.core.lsu_resp_valid

    pub load_err_o: &'a mut bool,           // datapath.core.lsu_load_err
    pub store_err_o: &'a mut bool,          // datapath.core.lsu_store_err

    pub busy_o: &'a mut bool,               // datapath.core.lsu_busy
}

pub struct CVE2LSU {

}

pub struct CVE2WriteBackInterface<'a> {
    pub en_wb_i: &'a bool,                  // datapath.core.rf_we_wb

    // pub instr_is_compress_id_i: &'a bool,   // datapath.core.instr_is_compressed_id
    pub rf_waddr_id_i: &'a u8,               // datapath.core.rf_waddr_id
    pub rf_wdata_id_i: &'a u32,              // datapath.core.rf_wdata_id
    pub rf_we_id_i: &'a bool,                // datapath.core.rf_we_id

    pub rf_wdata_lsu_i: &'a u32,             // datapath.core.rf_wdata_lsu
    pub rf_we_lsu_i: &'a bool,               // datapath.core.rf_we_lsu

    pub rf_waddr_wb_o: &'a mut u8,           // datapath.core.rf_waddr_wb
    pub rf_wdata_wb_o: &'a mut u32,          // datapath.core.rf_wdata_wb
    pub rf_we_wb_o: &'a mut bool,            // datapath.core.rf_we_wb

    pub lsu_resp_valid_i: &'a bool,          // datapath.core.lsu_resp_valid
    pub lsu_resp_err_i: &'a bool,            // datapath.core.lsu_resp_err
}

pub struct CVE2WriteBack {

}

pub struct CVE2RegisterFileInterface<'a> {
    pub raddr_a_i: &'a u8,                  // datapath.core.rf_raddr_a
    pub rdata_a_o: &'a mut u32,             // datapath.core.rf_rdata_a
    pub raddr_b_i: &'a u8,                  // datapath.core.rf_raddr_b
    pub rdata_b_o: &'a mut u32,             // datapath.core.rf_rdata_b
    pub waddr_a_i: &'a u8,                  // datapath.core.rf_waddr_wb
    pub wdata_a_i: &'a u32,                 // datapath.core.rf_wdata_wb
    pub we_a_i: &'a bool,                   // datapath.core.rf_we_wb
}

#[derive(Copy, Clone, Default)]
pub struct CVE2Core {
    pub instr_valid_id: bool,               // Whether the instruction in the ID stage is valid
    pub instr_new_id: bool,                 // Whether the instruction in the ID stage is new
    pub instr_rdata_id: u32,                // The instruction in the ID stage
    // pub instr_rdata_c_id: u16,              // The compressed instruction in the ID stage
    // pub instr_is_compressed_id: bool,       // Whether the instruction in the ID stage is compressed
    // pub instr_perf_count_id: bool,          // Whether the instruction in the ID stage is a performance counter instruction
    pub instr_fetch_err: bool,              // Whether an error occurred during instruction fetch
    // pub instr_fetch_err_plus2: bool,        // Whether an error occurred during instruction fetch for the next instruction
    // pub illegal_c_insn_id: bool,            // Whether an illegal compressed instruction was detected
    pub illegal_insn_id: bool,              // Whether an illegal instruction was detected
    
    pub pc_if: u32,                         // The program counter in the IF stage
    pub pc_id: u32,                         // The program counter in the ID stage
    pub imd_val_d_h_ex: [[bool; 2]; 2],     // The upper 2 bits of the inttermediate value output from ID stage for multi-cycle instructions
    pub imd_val_d_ex: [u32; 2],             // The intermediate value output from ID stage for multi-cycle instructions
    pub imd_val_q_h_ex: [[bool; 2]; 2],     // The upper 2 bits of the inttermediate value input to ID stage for multi-cycle instructions
    pub imd_val_q_ex: [u32; 2],             // The intermediate value input to ID stage for multi-cycle instructions
    pub imd_val_we_ex: [bool; 2],           // Whether the intermediate value is updated for multi-cycle instructions

    pub instr_first_cycle_id: bool,         // Whether the instruction in the ID stage is in the first cycle
    pub instr_valid_clear: bool,
    pub pc_set: bool,                       // Whether the program counter was updated in the ID stage
    pub pc_mux_id: PCSelect,                // The program counter mux select signal

    pub lsu_load_err: bool,                 // Whether an error occurred during a load operation
    pub lsu_store_err: bool,                // Whether an error occurred during a store operation

    // LSU signals
    pub lsu_addr_incr_req: bool,            // Whether the address incrementer is requested
    pub lsu_addr_last: u32,                 // The last address used by the address incrementer

    // Jump signals
    pub branch_target_ex: u32,              // The branch target address
    pub branch_decision: bool,              // The branch decision

    // Core busy signals
    pub ctrl_busy: bool,                    // Whether the core is busy
    pub if_busy: bool,                      // Whether the instruction fetch stage is busy
    pub lsu_busy: bool,                     // Whether the load/store unit is busy

    // Register file signals
    pub rf_raddr_a: u8,                     // The read address for register file port A (5-bit)
    pub rf_rdata_a: u32,                    // The read data from register file port A
    pub rf_raddr_b: u8,                     // The read address for register file port B (5-bit)
    pub rf_rdata_b: u32,                    // The read data from register file port B
    pub rf_ren_a: bool,                     // Whether the read port A is enabled
    pub rf_ren_b: bool,                     // Whether the read port B is enabled

    pub rf_waddr_wb: u8,                    // The write address for register file port WB (5-bit)
    pub rf_wdata_wb: u32,                   // The write data for register file port WB
    pub rf_we_wb: bool,                     // Whether the write port WB is enabled

    pub rf_wdata_lsu: u32,                  // The write data for register file port LSU
    pub rf_we_lsu: bool,                    // Whether the write port LSU is enabled

    pub rf_waddr_id: u8,                    // The write address for register file port ID (5-bit)
    pub rf_wdata_id: u32,                   // The write data for register file port ID
    pub rf_we_id: bool,                     // Whether the write port ID is enabled

    // ALU signals
    pub alu_operator_ex: ALUOperation,      // The ALU operation
    pub alu_operand_a_ex: u32,              // The first ALU operand
    pub alu_operand_b_ex: u32,              // The second ALU operand
    pub alu_adder_result_ex: u32,           // The result of the adder (forwarded to the LSU)
    pub result_ex: u32,                     // The result of the ALU operation

    // Data memory signals
    pub lsu_we: bool,                       // Whether the load/store unit is writing
    pub lsu_type: [bool; 2],                // The type of the load/store operation
    pub lsu_sign_ext: bool,                 // Whether the load/store unit is sign extending
    pub lsu_req: bool,                      // Whether the load/store unit is requested
    pub lsu_wdata: u32,                     // The data to be written by the load/store unit

    // stall control
    pub id_in_ready: bool,                  // Whether the ID stage is ready to accept new instructions
    pub ex_valid: bool,                     // Whether the EX stage is valid
    pub lsu_resp_valid: bool,               // Whether the LSU response is valid
    pub lsu_resp_err: bool,                 // Whether the LSU response is an error

    // Instruction memory signals
    pub instr_req_int: bool,                // Whether an instruction is being requested
}

#[derive(Copy, Clone, Default)]
pub struct CVE2Datapath {
    pub core: CVE2Core,
    pub if_stage: CVE2IFStage,
    pub id_stage: CVE2IDStage,

    pub boot_addr: u32,

    // Instruction memory interface
    pub instr_req_o: bool,          // Whether an instruction is being requested
    pub instr_gnt_i: bool,          // Whether the instruction read has been granted
    pub instr_rvalid_i: bool,       // Whether the read instruction is valid
    pub instr_addr_o: u32,          // The address of the instruction to be read
    pub instr_rdata_i: u32,         // The read instruction
    pub instr_err_i: bool,          // Whether an error occurred during the read

    // Data memory interface
    pub data_req_o: bool,           // Whether data is being requested
    pub data_gnt_i: bool,           // Whether the data read has been granted
    pub data_rvalid_i: bool,        // Whether the read data is valid
    pub data_we_o: bool,            // Whether data is being written
    pub data_be_o: [bool; 4],       // Byte enable (little-endian) for data write
    pub data_addr_o: u32,           // The address of the data to be read/written
    pub data_rdata_i: u32,          // The read data
    pub data_err_i: bool,           // Whether an error occurred during the read/write
}
