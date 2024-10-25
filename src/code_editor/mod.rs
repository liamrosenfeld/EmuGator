use dioxus::prelude::*;

mod monaco_editor;

use monaco::sys::editor::IEditorMinimapOptions;
use monaco_editor::MonacoEditor;

#[derive(Props, Clone, PartialEq)]
pub struct CodeEditorProps {}

/// A wrapper around the Monaco editor with our expected functionality
#[component]
#[allow(non_snake_case)]
pub fn CodeEditor(props: CodeEditorProps) -> Element {
    // basic model
    // TODO: tie this to external text
    let model = monaco::api::TextModel::create("test", None, None).unwrap();

    // basic options
    let options = monaco::api::CodeEditorOptions::default()
        .with_automatic_layout(true)
        .with_builtin_theme(monaco::sys::editor::BuiltinTheme::VsDark)
        .to_sys_options();

    // disable the minimap
    let disable_minimap = IEditorMinimapOptions::default();
    disable_minimap.set_enabled(Some(false));
    options.set_minimap(Some(&disable_minimap));

    rsx! {
        MonacoEditor {
            model: model,
            options: options,
        }
    }
}
