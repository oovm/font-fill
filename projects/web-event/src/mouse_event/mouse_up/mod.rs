use crate::wrapper::MouseEventWrapper;
use std::{
    fmt::{Debug, Formatter},
    ops::Deref,
};
use wasm_bindgen::JsCast;
use web_sys::Event;

/// - Bubbles: Yes
/// - Cancelable: Yes
/// - Event type: MouseEvent
/// - Supported HTML tags: All HTML elements, EXCEPT: `<base>`, `<bdo>`, `<br>`, `<head>`, `<html>`,
///   `<iframe>`, `<meta>`, `<param>`, `<script>`, `<style>`, and `<title>`.
#[derive(Clone)]
pub struct OnMouseUp {
    inner: MouseEventWrapper,
}

impl Debug for OnMouseUp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OnMouseUp")
    }
}

impl From<Event> for OnMouseUp {
    fn from(e: Event) -> Self {
        Self { inner: MouseEventWrapper(e.unchecked_into()) }
    }
}

impl Deref for OnMouseUp {
    type Target = MouseEventWrapper;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
