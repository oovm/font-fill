use js_sys::Array;
use std::fmt::{Debug, Formatter};
use wasm_bindgen::JsCast;
use web_sys::{DataTransfer, Event, EventTarget, InputEvent, Node, Window};

/// Execute when a user writes something in an `<input>` field.
/// - Bubbles: Yes
/// - Cancelable: No
/// - Event type: [`Event`] + [`InputEvent`]
/// - Supported HTML tags: 	`<input type="color">`, `<input type="date">`, `<input type="datetime">`,
///   `<input type="email">`, `<input type="month">`, `<input type="number">`, `<input
///   type="password">`, `<input type="range">`, `<input type="search">`, `<input type="tel">`,
///   `<input type="text">`, `<input type="time">`, `<input type="url">`, `<input type="week">` and
///   `<textarea>`.
#[derive(Clone, PartialEq, Eq)]
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

/// methods inherit form [`InputEvent`]
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

/// methods inherit form [`UiEvent`]
impl OnInput {
    /// Getter for the `view` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/view)
    #[inline]
    pub fn view(&self) -> Option<Window> {
        self.inner.view()
    }
    /// Getter for the `detail` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/detail)
    #[inline]
    pub fn detail(&self) -> i32 {
        self.inner.detail()
    }
    /// Getter for the `layerX` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerX)
    #[inline]
    pub fn layer_x(&self) -> i32 {
        self.inner.layer_x()
    }
    /// Getter for the `layerY` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerY)
    #[inline]
    pub fn layer_y(&self) -> i32 {
        self.inner.layer_y()
    }
    /// Getter for the `pageX` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageX)
    #[inline]
    pub fn page_x(&self) -> i32 {
        self.inner.page_x()
    }
    /// Getter for the `pageY` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageY)
    #[inline]
    pub fn page_y(&self) -> i32 {
        self.inner.page_y()
    }
    /// Getter for the `which` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/which)
    #[inline]
    pub fn which(&self) -> u32 {
        self.inner.which()
    }
    /// Getter for the `rangeParent` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeParent)
    #[inline]
    pub fn range_parent(&self) -> Option<Node> {
        self.inner.range_parent()
    }
    /// Getter for the `rangeOffset` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeOffset)
    #[inline]
    pub fn range_offset(&self) -> i32 {
        self.inner.range_offset()
    }
}

/// methods inherit form [`Event`]
impl OnInput {
    /// Getter for the `type` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/type)
    #[inline]
    pub fn r#type(&self) -> String {
        self.inner.type_()
    }
    /// Getter for the `target` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/target)
    pub fn target(&self) -> Option<EventTarget> {
        self.inner.target()
    }
    /// Getter for the `currentTarget` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget)
    pub fn current_target(&self) -> Option<EventTarget> {
        self.inner.current_target()
    }
    /// Getter for the `eventPhase` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/eventPhase)
    #[inline]
    pub fn event_phase(&self) -> u16 {
        self.inner.event_phase()
    }
    /// Getter for the `bubbles` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/bubbles)
    #[inline]
    pub fn bubbles(&self) -> bool {
        self.inner.bubbles()
    }
    /// Getter for the `cancelable` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelable)
    #[inline]
    pub fn cancelable(&self) -> bool {
        self.inner.cancelable()
    }
    /// Getter for the `defaultPrevented` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/defaultPrevented)
    #[inline]
    pub fn default_prevented(&self) -> bool {
        self.inner.default_prevented()
    }
    /// Getter for the `composed` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composed)
    #[inline]
    pub fn composed(&self) -> bool {
        self.inner.composed()
    }
    /// Getter for the `isTrusted` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/isTrusted)
    #[inline]
    pub fn is_trusted(&self) -> bool {
        self.inner.is_trusted()
    }
    /// Getter for the `timeStamp` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/timeStamp)
    #[inline]
    pub fn time_stamp(&self) -> f64 {
        self.inner.time_stamp()
    }
    /// Getter for the `cancelBubble` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)
    #[inline]
    pub fn cancel_bubble(&self) -> bool {
        self.inner.cancel_bubble()
    }
    /// Setter for the `cancelBubble` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)
    #[inline]
    pub fn set_cancel_bubble(&self, value: bool) {
        self.inner.set_cancel_bubble(value)
    }
    /// The `composedPath()` method.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath)
    #[inline]
    pub fn composed_path(&self) -> Array {
        self.inner.composed_path()
    }
    /// The `preventDefault()` method.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)
    #[inline]
    pub fn prevent_default(&self) {
        self.inner.prevent_default()
    }
    /// The `stopImmediatePropagation()` method.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopImmediatePropagation)
    #[inline]
    pub fn stop_immediate_propagation(&self) {
        self.inner.stop_immediate_propagation()
    }
    /// The `stopPropagation()` method.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation)
    #[inline]
    pub fn stop_propagation(&self) {
        self.inner.stop_propagation()
    }
}
