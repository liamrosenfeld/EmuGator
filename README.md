# EmuGator

Computer engineering capstone team project

## Completed Work
Team members who are not familiar with the Rust programming language have been focusing on learning Rust by working on exercises and reading documentation. We have also been working on creating a basic structure for the assembler and emulator to work off in the future. This includes determining how our data structures will be managed to allow emulating pipelining. Furthermore, we have been working on the user-interface.

# Project Architecture

## Monaco, Tailwind, and Dioxus Frontend
The architecture of the editor was designed and established during the pre-alpha stage. It consists of two key components:
- **Low-Level Component**: Manages adding and activating the Monaco editor to the DOM.
- **Top-Level Component**: Provides default configurations to improve appearance and expose the model of the editor to the Dioxus state life cycle.

Remaining architectural challenges include:
- Syntax highlighting
- Getting diagnostics from the assembler
- Implementing hover documentation

## EmuGator Core
The emulator is modeled as a finite state machine. Instructions are decoded according to the instruction formats defined in the [RV32I Base Integer Instruction Set Version 2.0](https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf). Each instruction resolves to an instruction handler that consumes the current state and produces the next state. This design allows the ability to replay states forwards or backwards, facilitating debugging of programs executed on the platform.

The emulator's internal state is modeled after the OpenHW CVE2 2-stage pipelined RISC-V processor. By reviewing the top-level SystemVerilog file, we extracted the CVE2's datapath and pipeline components to enable cycle-level debugging.

## RISC-V Assembler
The assembler is long yet simple and integrates with the frontend to take in assembly code as input, producing machine code output for the emulator.

### Key Features:
- The assembler operates on a two-pass system:
  1. **First Pass**: Looks at labels and memory sections (e.g., `.data` and `.text`), associating labels with memory addresses.
  2. **Second Pass**: Generates the actual machine code.

- The main function, `assemble`, handles the assembly string and coordinates the parsing passes.

A lot of the assembler code is redundant since every supported instruction requires defining components like `opcode`, `funct3`, and `funct7` in structures. Although the codebase is long, it remains straightforward.


## Known Bugs
- Proper communication between UI, emulator, and assembler have not been established.
- Assembler does not work for I-instructions that aren't the typical format such as FENCE
- Code is not properly tested. There are likely unnoticed bugs.

## Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Install Rust and Cargo: https://doc.rust-lang.org/cargo/getting-started/installation.html 
4. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command to install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080
