use std::convert::TryInto;
use std::error::Error;
use log::log;
use wasm_bindgen::{JsCast, JsValue};
use sycamore::builder::html::*;
use sycamore::prelude::*;
use sycamore::rt::Event;
use web_sys::{ClipboardEvent, DataTransfer, DataTransferItem, window};


#[component(Header < G >)]
pub fn header() -> View<G> {
    let name = Signal::new(String::new());
    let level = Signal::new(0u8);

    let paste_progress = PasteProgress {
        text: Some(upper),
        image: None,
    };

    h1().text("Hello ")
        .on("paste", cloned!(() => move |e:Event| { paste_progress.inject(e).ok(); }))
        .dyn_child(cloned!((name) => move || {
                    if *create_selector(cloned!((name) => move || !name.get().is_empty())).get() {
                        span()
                            .dyn_text(cloned!((name) => move || name.get().to_string()))
                            .build()
                    } else {
                        span().text("World").build()
                    }
                }))
        .text("!")
        .build()
}

///    let paste = (event.clipboardData || window.clipboardData).getData('text');
//     paste = paste.toUpperCase();
//
//     const selection = window.getSelection();
//     if (!selection.rangeCount) return false;
//     selection.deleteFromDocument();
//     selection.getRangeAt(0).insertNode(document.createTextNode(paste));
//
//     event.preventDefault();

pub type Result<T> = std::result::Result<T, EditorError>;

pub struct EditorError {
    kind: Box<EditorErrorKind>,
}

pub enum EditorErrorKind {}

pub struct PasteProgress {
    pub text: Option<fn(&str) -> String>,
    pub image: Option<fn(&str) -> String>,
}

impl PasteProgress {
    pub fn inject(&self, e: Event) -> Result<()> {
        let event = e.dyn_into::<ClipboardEvent>().unwrap();
        let data = event.clipboard_data().unwrap().items();
        let max = data.length();
        for i in 0..max {
            data.get(i).map(|f| self.inject_item(f));
        }
        return Ok(());
    }
    pub fn inject_item(&self, item: DataTransferItem) {
        match item.kind().as_str() {
            "string" => {
                match item.type_().as_str() {
                    "text/plain" => {
                        self.inject_text(item)
                    }
                    _ => log::info!("{:?}", item.type_()),
                }
            }
            // "file"
            _ => {
                // log::info!("{:?}", item.get_as_file());
                todo!()
            }
        }
    }
    fn inject_text(&self, item: DataTransferItem) {
        log::info!("{:?}", item.get_as_string(None));
    }
}


// pub fn paste_text_progress(filter: fn(&str) -> String, e: Event) -> Result<(), Box<dyn Error>> {
//
//
//     match data.get_data("text") {
//         Ok(o) => {
//             data.set_data("text", &filter(&o)).ok();
//             let selection = window().map(|f| f.get_selection());
//         }
//         Err(e) => {
//             log::warn!("{:#?}",e);
//         }
//     }
//     // let selection = window().get_or_insert();
//     event.prevent_default();
//     Ok(())
// }

pub fn upper(input: &str) -> String {
    input.to_ascii_uppercase()
}

#[component(Notedown < G >)]
pub fn notedown() -> View<G> {
    pre()
        .class("notedown")
        .bool_attr("contenteditable", true)
        .child(Header::create_component(()))
        .build()
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| view! { Notedown() });
}
