#import "@preview/charged-ieee:0.1.2": ieee
#import "@preview/wordometer:0.1.2": word-count, total-words

#show: ieee.with(
  title: [R1: EmuGator Design Plan],
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
  ),
  bibliography: bibliography("refs.bib"),
)

#show figure.where(kind: image): set figure(placement: none)
#show figure.where(kind: table): set figure(placement: none)
#show: word-count.with(exclude: (heading, figure))

= Introduction

== Purpose / Need

Teaching computer architecture at an introductory level is difficult. This is somewhat due to the amount of new content taught as compared to prior programming courses @Mirmotahari2003. Understanding the connection between assembly instructions, CPU behavior, and theory is difficult. EmuGator aims to fill this gap as a per-datapath-component emulator. Additionally, EmuGator will help students visualize how a CPU behaves during program execution by allowing students to set breakpoints and see data line values. EmuGator will be used as an educational emulator, which have shown to be helpful to students in introductory computer architecture classrooms @Patti2012. 

== Domain & Prior Art

Previously, new computer architectures have been developed for educational uses. For example, Little Computer 3 (LC-3) was developed and utilized in education @Liao2012. However, LC-3 is an architecture and does not provide a user-interface around it.

Some existing emulators include the Vercel application for online RISC V simulation @RiscVVercel. However, this application was built based on Kite @song_kite2019 and does not provide visuals as instructions move through the pipeline. Some computer science students at UCF created SWIM, an online MIPs and RISC V emulator and visualizer @Swim. However, this project also does not emulate a pipeline and its visualization is largely focused on which stage is currently active. Unlike these existing applications, Emugator will have enhanced visualization that communicates active components, data flow, and pipeline stages.

== Impact, Risk Assessment, & Limitations 

EmuGator aims to add to resources available for students learning computer organization, increase interest in assembly languages and decrease the difficulty in understanding them.

One risk of EmuGator is students becoming dependent on the visual components to work with assembly, making it more difficult to work with environments lacking these components. One ethical consideration is that EmuGator could mislead students, for example if they believed EmuGator's pipeline was the only pipeline.

To mitigate these risks, Emugator will encourage students and instructors to check out other forms of emulation after they have grown comfortable using EmuGator.

== Maintenance

Members of the group have decided to maintain the project beyond the time of this course. If it becomes too burdensome, a new committee of maintainers (potentially course staff using EmuGator) will be selected and/or the project will be open-sourced.

= Statement of Work

A list of several main features, approximate hours required, and who is interested in working on these features can be seen below. The total time before the prototype is estimated to be approximately 250 hours.

== Testing

Writing tests will be done in parallel to implementing features. Testing will be performed using Cargo's built in testing functionality, which can be run (locally or in CI) with `cargo test`.

Both the `assembler` and the `emulator` modules will have a `test` submodule with a collection of testing functions. Those testing functions will consist of unit tests to verify that particular instructions are behaving as expected and some integration tests checking that an external file is handled as expected. There will also be a `system-tests` top level module to test the connection between the assembler, emulator, and visualizer.

Because the text editor is largely built upon controlling external JavaScript, it will not be possible to write comprehensive cargo tests. So tests will be limited to testing particular business logic and manually testing the website.

== Environment Setup

All members will set up their local environment. Given the meetings necessary, potential inconsistencies between platforms, and general issues that arise, 10 hours per member has been allocated for environment setup, for a total of *50 hours*. This should be completed by *October 18th*. No members other than Liam Rosenfeld currently know Rust. Learning the language as necessary for the project will require 10 hours each, for a total of *40 hours* and should also be completed before *October 18th*.

== RISC-V Assembly Text Editor

The RISC-V assembly text editor will be responsible for taking in a RISC-V assembly file, basic functionality such as saving the file and setting it as input for an assembler, and syntax highlighting. Creating a text editor from scratch would be far out of scope, so the Monaco editor will be used as the foundation of the editing functionality. Integrating this editor will take approximately *30 hours* and will be done by Liam Rosenfeld. This should be completed before the week of *November 1st*.

A *secondary* feature for this component is hover tooltips. Hovering over an instruction would explain what the instruction does.

== RISC-V Assembler

The assembler should start with a strict subset of commands and be built in the Rust programming language and integrated with web assembly. An assembler capable of handling commands in the RV32I subset of RISC-V and outputting the machine code should be created by Christopher Tressler before *November 1st* and should take approximately *10 hours*. It should also be able to provide readable error messages for invalid assembly to be displayed by the text editor. 

== RISC-V Emulator

The emulator will take in the machine code produced by the assembler and perform operations on a model of a real RISC-V processor, producing outputs for every clock cycle. This will be able to run in both pipelining and non-pipelining modes. The emulator will contain a memory section for both data and text. Rohan Simon and Fiona Cahalan. will work for 20 hours on this, totalling *40 hours* and it should be completed by *November 7th*.

== Debugger

The debugger will consist of a program able to set breakpoints and step through both instructions and individual clock cycles. The debugger will interact with the memory display and CPU pipeline visualization utility. This debugger will be able to stop the program based on user input. This section will be led by Nikhil Iyer and will take approximately *30 hours* before *November 7th*. Furthermore, due to the interactive nature of the debugger with other features, all other team members have been allocated 5 hours this semester for specification verification with Nikhil, for a total of *20 hours*.

The debugger will also require a front-end display. Creating appealing CPU graphics for this and integrating with the front end will be done by Nikhil and Liam and will take approximately 15 hours each for a total of *30 hours*. This should be completed by *November 14th*.

== Memory Display & I/O

This will be a display for the memory on the front end of the web application. This will be done in Rust through the Dioxus front-end by Christopher Tressler. This will take approximately *10 hours* and should be completed by *November 14th*

A *secondary* feature would be to add support for I/O for a specific RISC-V processor chosen. This would be a stretch feature, and would not be completed until late in CpE Design 2 if at all.

== Instruction Machine Code Display

This will be a display for the assembled machine code on the front end of the web application. This will be done in Rust through the Dioxus front-end by Christopher Tressler, should take *10 hours* and be done by *November 14th*.

== CPU Visualization

The CPU visualization will be split into two parts: back-end and front-end.

Those involved in the back-end of this solution will need to ensure that during emulation CPU component states are tracked, consistent, and fetchable by the front-end. Integrating this feature into the emulator will assigned to Fiona Cahalan and Rohan Simon, and should take approximately 10 hours each for a total of *20 hours* and be completed by *November 14th*.

A front-end is also required for the CPU visualization. Christopher Tressler, Liam Rosenfeld, and Nikhil Iyer will work on the front end each contributing approximately 10 hours for a total of *30 hours* to be done before *November 22nd*.

A *secondary* feature for this to be completed would be to color code where instructions are in the pipeline, registers, memory, etc. simultaneously and to color-code parts of the instruction (ex. opcode) to match parts in the datapath.

== Milestone 1: CpE Design 1 Prototype
The first prototype should be completed before *November 26th*, with a limited subset of instructions and functionality for a full RISC-V processor but enough to demonstrate the practical use of the application. The visualization will additionally be of a more abstract model of a data path. When the more detailed processor visualization is completed, the more abstract model will still be available as a mode.

The single performance expectation would be that users can upload, assemble, and emulate in a reasonable amount of time. This should process in real-time using web assembly.

@features-table summarizes the features, due date, time allocated for them, and who is assigned to work on them. 

#figure(
  table(
    columns: 4,
    [*Features*], [*Assigned*], [*Time*], [*Due*],
    [Environment setup],[everyone],[50],[Oct. 18],
    [Learn Rust],[Fiona, Nikhil, Rohan, Christopher],[40],[Oct. 18],
    [Text editor],[Liam],[30],[Nov. 1],
    [Assembler],[Christopher],[10],[Nov. 1],
    [Emulator],[Fiona and Rohan],[40],[Nov. 7],
    [Debugger],[Nikhil and Liam],[30],[Nov. 7],
    [Debugger front-end],[Nikhil and Liam],[30],[Nov. 14],
    [Memory display],[Christopher],[10],[Nov. 14],
    [Machine code display],[Christopher],[10],[Nov. 14],
    [CPU visualization],[Fiona and Rohan],[20],[Nov. 14],
    [CPU visualization front-end],[Christopher, Nikhil, Liam],[30],[Dec. 22],
  ),
  caption: [Table of features, assignments, and due dates.]
) <features-table>

= Deliverable Artifacts
The primary deliverable will be the EmuGator webpage, running the simulator and allowing users to access it via a simple URL. However, the source code for EmuGator will be available for viewing in a public GitHub repository. The GitHub repository will also contain documentation and examples showcasing the features of EmuGator. 

= Mockups

== Interfaces

Users will interface with EmuGator with any common web browser, using a laptop or desktop.
#figure(
  image("img/interface.png", width: 50%),
  caption: "Keyboard, monitor, and mouse for interfacing with EmuGator."
)<interface>

== Systems

#figure(
  image("img/systems.png"),
  caption: "The overall flow of data between indenpendent components of the project.",
)<storyboard>

== Networking
EmuGator will have basic server to server communication and server to web assembly files and a HTML file to run it. 
#figure(
  image("img/networking.png"),
  caption: "Model of communication."
)<networking>

== Storyboards

EmuGator will have one page with all views visible to maintain simplicity, and avoid users needing to switch between tabs frequently to cross reference data.

#figure(
  image("img/storyboard.png"),
  caption: "Mockup of main page.",
)<storyboard>

== Draft Schematics

No hardware involved.

= Word Count
// #show: word-count

#total-words

#colbreak(weak: true)