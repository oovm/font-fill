use std::fmt::{Debug, Formatter};
use wasm_bindgen::JsCast;
use web_sys::{DataTransfer, Event, EventTarget, InputEvent};

/// Execute when a user writes something in an <input> field.
/// - Bubbles: Yes
/// - Cancelable: No
/// - Event type: [`InputEvent`]
/// - Supported HTML tags: 	`<input type="color">`, `<input type="date">`, `<input type="datetime">`,
///   `<input type="email">`, `<input type="month">`, `<input type="number">`, `<input
///   type="password">`, `<input type="range">`, `<input type="search">`, `<input type="tel">`,
///   `<input type="text">`, `<input type="time">`, `<input type="url">`, `<input type="week">` and
///   `<textarea>`.
#[derive(Clone)]
pub struct OnInput {
    inner: InputEvent,
}

impl Debug for OnInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OnInput")
    }
}

impl From<Event> for OnInput {
    fn from(e: Event) -> Self {
        Self { inner: e.unchecked_into() }
    }
}

impl OnInput {
    /// Getter for the `isComposing` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/isComposing)
    #[inline]
    pub fn is_composing(&self) -> bool {
        self.inner.is_composing()
    }
    /// Getter for the `inputType` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/inputType)
    #[inline]
    pub fn input_type(&self) -> String {
        self.inner.input_type()
    }
    /// Getter for the `data` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data)
    #[inline]
    pub fn data(&self) -> Option<String> {
        self.inner.data()
    }
    /// Getter for the `dataTransfer` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/dataTransfer)
    #[inline]
    pub fn data_transfer(&self) -> Option<DataTransfer> {
        self.inner.data_transfer()
    }
    /// The `getTargetRanges()` method.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/getTargetRanges)
    #[inline]
    pub fn get_target_ranges(&self) -> ::js_sys::Array {
        self.inner.get_target_ranges()
    }
}
