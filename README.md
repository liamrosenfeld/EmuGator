# EmuGator

Computer engineering capstone team project

## Completed Work
Team members who are not familiar with the Rust programming language have been focusing on learning Rust by working on exercises and reading documentation. We have also been working on creating a basic structure for the assembler and emulator to work off in the future. This includes determining how our data structures will be managed to allow emulating pipelining. Furthermore, we have been working on the user-interface.

## Project Architecture
The user interface is being built using Dioxus. Dioxus interacts with the assembler, which takes in input as a string and returns ordered maps to the emulator. The emulator will have an instruction handler that will generate diffs on the pipeline state.

## Known Bugs
- Proper communication between UI, emulator, and assembler have not been established.
- Assembler does not work for I-instructions that aren't the typical format
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
