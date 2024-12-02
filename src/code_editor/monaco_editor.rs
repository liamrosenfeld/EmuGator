use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use monaco::{
    api::{CodeEditor as MonacoController, TextModel},
    sys::{
        editor::{
            IModelDecorationOptions, IModelDeltaDecoration, IStandaloneEditorConstructionOptions,
        },
        IRange, Range,
    },
};
use wasm_bindgen::{JsCast, JsValue};

#[derive(Clone, PartialEq, Debug)]
pub struct LineHighlight {
    pub line: usize,
    pub css_class: &'static str,
}

/// The monaco editor directly wrapped
#[component]
#[allow(non_snake_case)]
pub fn MonacoEditor(
    options: ReadOnlySignal<Option<IStandaloneEditorConstructionOptions>>,
    model: ReadOnlySignal<Option<TextModel>>,
    line_highlights: ReadOnlySignal<Vec<LineHighlight>>,
) -> Element {
    let mut editor = use_signal::<Option<MonacoController>>(|| None);
    let element_id = "monaco-editor";

    let mut curr_decorations = use_signal(|| js_sys::Array::new());

    // create editor
    use_effect(move || {
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(element_id))
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let options = options.read().deref().clone();
            *editor.write() = Some(MonacoController::create(&el, options));
        }
    });

    // handle model changes
    use_effect(move || {
        if let Some(editor_instance) = editor.write().as_mut() {
            // Update model if changed
            if let Some(model) = &*model.read() {
                editor_instance.set_model(model);
            }
        }
    });

    // handle line highlight changes
    use_effect(move || {
        if let Some(editor_instance) = editor.write().as_mut() {
            if let Some(model) = editor_instance.get_model().as_ref() {
                // find new highlights
                let new_decor = js_sys::Array::new();
                info!("line_highlights: {:?}", line_highlights.read());
                for line_highlight in line_highlights.read().iter() {
                    new_decor.push(&line_decoration(
                        line_highlight.line,
                        line_highlight.css_class,
                    ));
                }

                // apply highlights
                let applied =
                    model
                        .as_ref()
                        .delta_decorations(&curr_decorations.peek(), &new_decor, None);

                // store highlights for next delta
                *curr_decorations.write() = applied;
            }
        }
    });

    rsx! {
        div { id: element_id, style: "width: 100%; height: 100%;" }
    }
}

// Example usage function
pub fn line_decoration(line_number: usize, color: &'static str) -> IModelDeltaDecoration {
    let decoration: IModelDeltaDecoration = new_object().into();
    let range = Range::new(line_number as f64, 0.0, line_number as f64, 1.0);
    decoration.set_range(&IRange::from(range.dyn_into::<JsValue>().unwrap()));

    let options: IModelDecorationOptions = new_object().into();
    options.set_is_whole_line(Some(true));
    options.set_z_index(Some(9999.0));
    options.set_class_name(Some(color));

    decoration.set_options(&options);

    decoration.into()
}

// Creates a new `JsValue`. Done for convenience and readability.
fn new_object() -> JsValue {
    js_sys::Object::new().into()
}
