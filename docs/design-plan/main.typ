#import "@preview/charged-ieee:0.1.2": ieee
#import "@preview/wordometer:0.1.2": word-count, total-words

#show: ieee.with(
  title: [R1: EmuGator Design Draft],
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

It has been established that teaching computer architecture at an introductory level is difficult. This is partially due to the separation between prior programming education and the new content being taught @Mirmotahari2003. For students, understanding the connection between assembly instructions in programs, a CPU’s behavior, and theory can be difficult. EmuGator aims to fill this gap as a CPU emulator that emulates each component in the data path. Additionally, EmuGator will help students to visualize how a CPU behaves while running a program by allowing students to set breakpoints and see what values data lines are set to. EmuGator will be an educational resource for students. CPU emulators are helpful to students, as shown by research on the use of the EduMIPS64 simulator in introductory computer architecture courses which found the simulator helpful for learning @Patti2012. 

== Domain & Prior Art

Previously, new computer architectures have been developed for educational uses in computer organization. For example, Little Computer 3 (LC-3) was developed and utilized in education and microprocessor labs @Liao2012. However, LC-3 is an architecture and in itself does not provide a specific user interface or tool-chain. 

There are a collection of existing simple CPU emulators online. For example, there is a Vercel application for online RISC V simulation @RiscVVercel. However, this application was built based on Kite @song_kite2019 and does not provide a visual representation of instructions as they go through a pipeline. Some computer science students at UCF created SWIM, an online MIPs and RISC V emulator and visualizer @Swim. However, this project also does not emulate a pipeline and its visualization is largely focused on which stage is currently active. EmuGator will target educational audiences, but unlike these existing applications it will have enhanced visualization that clearly communicates active components, data flow, and pipeline stages.

== Impact, Risk Assessment, & Limitations 

EmuGator contributes to the resources available for students learning computer organization, serving to increase interest in assembly languages and computer organization while decreasing the difficulty of understanding such material. However one potential drawback is the risk of students becoming dependent on the visual components of EmuGator to work with assembly, hampering a student’s ability to move onto environments that do not have versatile visual representations. One ethical consideration would be if EmuGator could misled or confuse students, resulting in a worse understanding of computer architecture and decreased interest. 

= Statement of Work

This project requires several main features to be completed. A list of several features, approximate hours required, and who is interested in working on these features can be seen below. The total time before the prototype is estimated to be approximately 250 hours.

== Environment Setup

All members involved with the project will be required to setup their own local environment. Given the meetings necessary, potential inconsistencies between platforms, and general issues that arise, 10 hours per member has been allocated for environment setup, for a total of *50 hours*. This should be completed by *October 18th*. No members other than Liam Rosenfeld currently know Rust. Learning the language as necessary for the project will require 10 hours each, for a total of *40 hours*. Every member should be comfortable with Rust before *October 18th*.

== RISC-V Assembly Text Editor

The RISC-V assembly text editor will be responsible for taking in a RISC-V assembly file, basic functionality such as saving the file and setting it as input for an assembler, and syntax highlighting. Creating a text editor from scratch would be an endeavor of scope deserving its own capstone project, so the Monaco editor will be used as the foundation of the editing functionality. Integrating this editor will take approximately *30 hours* and will be done by Liam Rosenfeld. This should be completed before the week of *November 1st*.

A *secondary* feature for this component is hover tooltips. Hovering over an instruction would show an explanation for that the instruction does.

== RISC-V Assembler

The assembler should start with a strict subset of commands and be built in the Rust programming language to be later be integrated with web assembly. An assembler capable of handling commands in the RV32I subset of RISC-V and outputting the machine code should be created and should take approximately *10 hours*. It should also be able to provide readable error messages for invalid assembly to be displayed by the text editor. This will be created by Christopher Tressler and should be completed by *November 1st*. 

== RISC-V Emulator

The emulator will take in the machine code produced by the assembler and perform operations on a model of a real RISC-V processor, producing outputs for every clock cycle. This will be able to run in both pipelining and non-pipelining modes. The emulator will contain a memory section for both data and text. Rohan Simon and Fiona Cahalan. Each member is expected to work for 20 hours on this, totalling *40 hours*. This should be completed by *November 7th*.

== Debugger

The debugger will consist of a program able to set breakpoints and step through both instructions and individual clock cycles. The debugger will interact with the memory display and CPU pipeline visualization utility. This debugger will be able to stop the program based on user input. This section will be led by Nikhil Iyer and will take approximately *30 hours*. The debugger should be completed before *November 7th*. Furthermore, due to the interactive nature of the debugger with other features, all other team members have been allocated 5 hours this semester for specification verification with Nikhil, for a total of *20 hours*.

The debugger will also require a front-end display. Creating appealing CPU graphics for this and integrating with the front end will be done by Nikhil and Liam and will take approximately 15 hours each for a total of *30 hours*. This should be completed by *November 14th*.

== Memory Display & I/O

This will be a display for the memory on the front end of the web application. This will be done in Rust through the Dioxus front-end by Christopher Tressler. This will take approximately *10 hours* and should be completed by *November 14th*

A *secondary* feature would be to add support for I/O for a specific RISC-V processor chosen. This would be a stretch feature, and would not be completed until late in CpE Design 2 if at all.

== Instruction Machine Code Display

This will be a display for the assembled machine code on the front end of the web application. This will be done in Rust through the Dioxus front-end by Christopher Tressler. This will take approximately *10 hours* and should be completed by *November 14th*.

== CPU Visualization

The CPU visualization will be split into two parts: back-end and front-end.

Those involved in the back-end of this solution will need to ensure that during emulation CPU component states are tracked, consistent, and fetchable by the front-end. Integrating this feature into the emulator will assigned to Fiona Cahalan and Rohan Simon, and should take approximately 10 hours each for a total of *20 hours* and should be completed by *November 14th*.

A front-end is also required for the CPU visualization. Christopher Tressler, Liam Rosenfeld, and Nikhil Iyer will work on the front end each contributing approximately 10 hours for a total of *30 hours* and this should be completed before *December 22nd*.

A *secondary* feature for this to be completed would be to color code where instructions are in the pipeline, registers, memory, etc. simultaneously. We could also color code parts of the instruction itself such as the opcode, registers, and values to match up with the datapath.

== Milestone 1: CpE Design 1 Prototype
The first prototype should be completed before *November 26th*, with a limited subset of instructions and functionality for a full RISC-V processor but enough to demonstrate the practical use of the application. The visualization will additionally be of a more abstract model of a data path. When the more detailed processor visualization is completed, the more abstract model will still be available as a mode.

The single performance expectation would be that users can upload, assemble, and emulate in a reasonable amount of time. This should process in real-time and will be solved by using web-assembly in app.

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

EmuGator will have one page with all views visible to maintain simplicity, and avoiding users needing to switch between tabs frequently to cross reference data.

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