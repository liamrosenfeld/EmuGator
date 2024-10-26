#import "@preview/charged-ieee:0.1.2": ieee
#import "@preview/wordometer:0.1.2": word-count, total-words

#show: ieee.with(
  title: [M1: EmuGator Pre Alpha],
  authors: (
    (
      name: "Fiona Cahalan",
    ),
    (
      name: "Nikhil Iyer",
    ),
    (
      name: "Liam Rosenfeld"
    ),
    (
      name: "Rohan Simon"
    ),
    (
      name: "Christopher Tressler"
    )
  )
)

#show figure.where(kind: image): set figure(placement: none)
#show figure.where(kind: table): set figure(placement: none)
#show: word-count.with(exclude: (heading, figure))
#show link: underline

= Summary
Most of the team was not familiar with Rust before starting this project. Consequently, those team members have dedicated time to learning Rust. Focusing on learning Rust aims to improve code quality, reducing the need to rewrite poorly written code in the future. However, we also worked on embedding a basic Monaco editor into the user interface, structuring the assembler, and outlining the basic data structures of the emulator.

= Architecture

== Monaco, Tailwind, and Dioxus Frontend
The architecture of the editor was designed and established during pre alpha. It consists of two components. The low level component manages adding and activating the Monaco editor to the DOM. The top level components provides default configurations to improve appearance and expose the model of the editor to the Dioxus state life cycle. Remaining architectural problems to solve include syntax highlighting, getting diagnostics from the assembler, and hover documentation.

== EmuGator Core
The emulator is modeled as a finite state machine. Instructions are decoded according to the instruction formats defined in the #link("https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf")[RV32I Base Integer Instruction Set Version 2.0]. 
Instructions resolve to an instruction handler which consumes the current state and produces the next state. This means we will be able to replay states forwards or backwards when debugging programs executed on the platform.

The emulator internal state is modeled on the OpenHW CVE2 2-stage pipelined RISC-V processor. By reviewing the top-level SystemVerilog file, we were able to extract the CVE2's datapath and pipeline components to allow cycle-level debug.

== RISC-V Assembler
The assembler is long yet simple. It is built to work with the front-end as input and send an appropriate output to the emulator. 

The assembler works using a two-pass system. In the implemented assembler, the main function is "assemble" which takes in the assembly string. This function then performs two parsing passes. The first pass is to look at labels and memory section (i.e. `.data` and `.text`). It associates labels with memory addresses. Afterwards, it has a second pass which generates the actual machine code.

A lot of the code in the assembler is redundant, since for every instruction supported information like opcode, funct3, and funct7 needed to be defined in structures. Hence the code is long but not complicated.

= Information Handling

== Communication
The embedded Monaco editor exposes its content, the user's assembly code, as a string that is consumed by the assembler. 

The assembler consumes the string input from the user and returns 3 maps: the instruction memory map (TEXT), the data memory map (DATA), and the source map.

These 3 maps are consumed by the emulator which decodes the instructions from the instruction memory map and executes them against the data memory map. The source map enables the debugging system to associate a given position in the binary with the line of source code that emitted it.

== Integrity & Resilience
The assembler does a plethora of error checking. For example, if a user inputs invalid registers, using an invalid mnemonic, or uses a valid mnemonic with an incorrect format, the assembler will output a detailed message (for example: `"Invalid numeric value: value_here"`).

The emulator's instruction decode step includes error handling for both malformed instructions and unknown opcodes in case the output from the assembler is corrupted or incorrect.

= Experimentation, Failures, Difficulties

== Editor

The editor went through a large amount of experimentation before settling on the current architecture. The initial prototype used a lot of wrapper types for holding JavaScript objects between renders, but this led to types that were 6 generics deep and quite unruly to do manage. It later became apparent that this was a re-implementation of the behavior available via Dioxus's builtin `Signal<T>` type, so moving over to that was able to simplify a lot.

The other editor experimentation involved connecting Monaco's closure based system of tracking changes to Dioxus's value based system. The primary challenge there was creating a map between the two without introducing circular dependencies which would result in an infinite loop. A solution of manually dirtying an intermediate hook was eventually reached.

== Learning Rust
For some team members, learning Rust has been difficult. Particularly, the jump from C and C++ to Rust is a steep learning curve. Completing small Rust programming exercises have been helpful.

== Architecture
Rust has unique semantic elements not present in other languages such as ownership and lifetime. These constructs allow Rust to statically detect many errors which typically surface at runtime in C/C++ however, they also make software design more complex as all code must be compliant. For example, instructions in a pipeline processor execute in "parallel" but only one instruction handler may hold the ownership (write access) of the emulator's state as a time. 

= Research
Liam researched Rust libraries to use for building the front end user interface, considering egui, yew, and dioxus, along with general requirements to target web assembly. The emulator team has been researching how to best emulate pipelining, considering whether to use an emulator state or data path state. The emulator team decided to emulate the OpenHW group's CORE-V CVE2 CPU because it has a simple two stage pipeline and would also allow us to acquire the physical hardware in the future, if desired.


= Relevant Links
- #link("https://github.com/liamrosenfeld/EmuGator")[Project Repository]
- #link("https://docs.google.com/spreadsheets/d/1_vsUU2aLrvSd0vPCdF1d4ZKzKjJFwAKDvYvVKObyMks/edit?usp=sharing")[Time Log Google Sheet]
- #link("https://www.youtube.com/watch?v=w9rLU30XIfg")[Video]
- #link("https://github.com/liamrosenfeld/EmuGator/releases/tag/pre-alpha")[Built Binary]
