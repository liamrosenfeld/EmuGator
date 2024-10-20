# EmuGator

Computer engineering capstone team project

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
