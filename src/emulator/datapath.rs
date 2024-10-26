/// Struct representing the datapath for the `cve2_top` module.
/// Taken from https://github.com/openhwgroup/cve2/blob/main/rtl/cve2_top.sv

#[derive(Clone, Copy)]
pub struct CVE2Datapath {
    // Clock and Reset
    clk_i: bool,  // Input clock signal.
    rst_ni: bool, // Active-low reset signal.

    // Instruction memory interface
    instr_req_o: bool,    // Output signal requesting an instruction fetch.
    instr_addr_o: u32,    // Output address for fetching instructions.
    instr_rdata_i: u32,   // Input data received as the fetched instruction.
    instr_gnt_i: bool,    // Input signal indicating the instruction request is granted.
    instr_rvalid_i: bool, // Input signal indicating valid instruction data is available.
    instr_err_i: bool,    // Input signal indicating an error during instruction fetch.

    // Data memory interface
    data_req_o: bool,    // Output signal requesting a data memory operation.
    data_addr_o: u32,    // Output address for the data memory operation.
    data_wdata_o: u32,   // Output data to be written to memory.
    data_rdata_i: u32,   // Input data read from memory.
    data_we_o: bool,     // Output write-enable signal for data memory.
    data_be_o: u8,       // Output byte-enable (4-bit) for selective byte access in 32-bit words.
    data_gnt_i: bool,    // Input signal indicating the data request is granted.
    data_rvalid_i: bool, // Input signal indicating valid data is available.
    data_err_i: bool,    // Input signal indicating an error during the data memory operation.

    // Core execution control signals
    fetch_enable_i: bool, // Input signal enabling instruction fetch.
    core_sleep_o: bool,   // Output signal indicating if the core is in sleep mode.

    // Interrupt inputs
    irq_software_i: bool, // Input software interrupt request signal.
    irq_timer_i: bool,    // Input timer interrupt request signal.
    irq_external_i: bool, // Input external interrupt request signal.
    irq_fast_i: u16,      // Input fast interrupt vector, 16 bits for fast IRQs.
    irq_nm_i: bool,       // Input non-maskable interrupt request signal.

    // Debug Interface
    debug_req_i: bool, // Input signal indicating a debug request.
}

pub struct CVE2Pipeline {
    IF: u32, // Instruction Fetch Buffer
    ID: u32, // Instruction Decode Buffer
    datapath: CVE2Datapath,
}

impl CVE2Pipeline {
    fn fillIF(&mut self) {
        // Fill the IF buffer if the instruction was requested and granted
        if self.datapath.instr_gnt_i && self.datapath.instr_req_o {
            self.IF = self.datapath.instr_rdata_i;
        }
    }

    fn fillID(&mut self) {
        // Fill the ID buffer if the instruction is valid
        if self.datapath.instr_rvalid_i {
            self.IF = self.datapath.instr_rdata_i;
        }
    }
}
