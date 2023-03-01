mod inherit_event;
mod inherit_ui_event;
mod inherit_mouse_event;
mod inherit_pointer_event;

use js_sys::Array;
use std::fmt::{Debug, Formatter};
use wasm_bindgen::JsCast;
use web_sys::{Event, PointerEvent};
use web_sys::EventTarget;

/// - Bubbles: Yes
/// - Cancelable: Yes
/// - Event type: [`Event`] :> [`UiEvent`] :> [`MouseEvent`] :> [`PointerEvent`]
/// - Supported HTML tags: All HTML elements, EXCEPT: `<base>`, `<bdo>`, `<br>`, `<head>`, `<html>`,
///   `<iframe>`, `<meta>`, `<param>`, `<script>`, `<style>`, `<title>`
#[derive(Clone, PartialEq, Eq)]
pub struct OnClick(PointerEvent);

impl Debug for OnClick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OnClickEvent")
    }
}

impl From<Event> for OnClick {
    fn from(e: Event) -> Self {
        Self(e.unchecked_into())
    }
}

