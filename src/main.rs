#[allow(non_snake_case, unused)]
mod assembler;
mod code_editor;
mod emulator;
mod isa;
mod utils;
mod interface;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use interface::App;

const _TAILWIND_URL: &str = manganis::mg!(file("./assets/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    code_editor::register_riscv_language();
    launch(App);
}
