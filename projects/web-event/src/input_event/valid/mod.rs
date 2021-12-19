use std::fmt::{Debug, Formatter};
use wasm_bindgen::JsCast;
use web_sys::Event;

/// Execute when a user writes something in an `<input>` field.
/// - Bubbles: No
/// - Cancelable: Yes
/// - Event type: [`Event`]
/// - Supported HTML tags: 	`<input>`.
#[derive(Clone)]
pub struct OnInvalid {
    inner: Event,
}

impl Debug for OnInvalid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OnInput")
    }
}

impl From<Event> for OnInvalid {
    fn from(e: Event) -> Self {
        Self { inner: e.unchecked_into() }
    }
}
