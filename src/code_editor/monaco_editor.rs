use dioxus::prelude::*;
use monaco::{
    api::{CodeEditor as MonacoController, TextModel},
    sys::editor::IStandaloneEditorConstructionOptions,
};
use wasm_bindgen::JsCast;

#[derive(Props, Clone, PartialEq)]
pub struct MonacoProps {
    pub options: Option<IStandaloneEditorConstructionOptions>,
    pub model: Option<TextModel>,
}

/// The monaco editor directly wrapped
#[component]
#[allow(non_snake_case)]
pub fn MonacoEditor(props: MonacoProps) -> Element {
    let mut editor = use_signal::<Option<MonacoController>>(|| None);
    let element_id = "monaco-editor";

    // create editor
    use_effect(move || {
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(element_id))
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
        {
            *editor.write() = Some(MonacoController::create(&el, props.options.clone()));
        }
    });

    // handle model changes
    use_effect(move || {
        if let Some(editor_instance) = editor.write().as_mut() {
            // Update model if changed
            if let Some(model) = &props.model {
                editor_instance.set_model(model);
            }
        }
    });

    rsx! {
        div {
            id: element_id,
            style: "width: 100%; height: 100%;",
        }
    }
}
