mod inherit_event;
mod inherit_ui_event;
mod inherit_mouse_event;

use std::{
    fmt::{Debug, Formatter},
};
use wasm_bindgen::JsCast;
use web_sys::{Event, MouseEvent};
use web_sys::EventTarget;



/// - Bubbles: Yes
/// - Cancelable: Yes
/// - Event type: MouseEvent
/// - Supported HTML tags: All HTML elements, EXCEPT: `<base>`, `<bdo>`, `<br>`, `<head>`, `<html>`,
///   `<iframe>`, `<meta>`, `<param>`, `<script>`, `<style>`, and `<title>`.
#[derive(Clone, PartialEq, Eq)]
pub struct OnMouseUp(MouseEvent);

impl Debug for OnMouseUp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OnMouseUp")
    }
}

impl From<Event> for OnMouseUp {
    fn from(e: Event) -> Self {
        Self(e.unchecked_into())
    }
}
