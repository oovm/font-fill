use std::fmt::{Debug, Formatter};
use wasm_bindgen::JsCast;
use web_sys::{DataTransfer, Event, EventTarget, InputEvent, UiEvent};

/// Execute when a user writes something in an `<input>` field.
/// - Bubbles: No
/// - Cancelable: Yes
/// - Event type: [`Event`] + [`UiEvent`]
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

impl OnInvalid {
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
