use std::fmt::{Debug, Formatter};
use js_sys::Array;
use wasm_bindgen::JsCast;
use web_sys::{Event, PointerEvent};

/// - Bubbles: Yes
/// - Cancelable: Yes
/// - Event type: MouseEvent
/// - Supported HTML tags: All HTML elements, EXCEPT: `<base>`, `<bdo>`, `<br>`, `<head>`, `<html>`,
///   `<iframe>`, `<meta>`, `<param>`, `<script>`, `<style>`, `<title>`
#[derive(Clone)]
pub struct OnClick {
    inner: PointerEvent,
}

impl Debug for OnClick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OnClickEvent")
    }
}

impl From<Event> for OnClick {
    fn from(e: Event) -> Self {
        Self { inner: e.unchecked_into() }
    }
}

impl OnClick {
    /// Getter for the `pointerId` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerId)
    #[inline]
    pub fn pointer_id(&self) -> i32 {
        self.inner.pointer_id()
    }
    /// Getter for the `width` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/width)
    #[inline]
    pub fn width(&self) -> i32 {
        self.inner.width()
    }
    /// Getter for the `height` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/height)
    #[inline]
    pub fn height(&self) -> i32 {
        self.inner.height()
    }
    /// Getter for the `pressure` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure)
    #[inline]
    pub fn pressure(&self) -> f32 {
        self.inner.pressure()
    }
    /// Getter for the `tangentialPressure` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tangentialPressure)
    #[inline]
    pub fn tangential_pressure(&self) -> f32 {
        self.inner.tangential_pressure()
    }
    /// Getter for the `tiltX` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltX)
    #[inline]
    pub fn tilt_x(&self) -> i32 {
        self.inner.tilt_x()
    }
    /// Getter for the `tiltY` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltY)
    #[inline]
    pub fn tilt_y(&self) -> i32 {
        self.inner.tilt_y()
    }
    /// Getter for the `twist` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/twist)
    #[inline]
    pub fn twist(&self) -> i32 {
        self.inner.twist()
    }
    /// Getter for the `pointerType` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerType)
    #[inline]
    pub fn pointer_type(&self) -> String {
        self.inner.pointer_type()
    }
    /// Getter for the `isPrimary` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/isPrimary)
    #[inline]
    pub fn is_primary(&self) -> bool {
        self.inner.is_primary()
    }
    /// The `getCoalescedEvents()` method.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getCoalescedEvents)
    #[inline]
    pub fn get_coalesced_events(&self) -> Array {
        self.inner.get_coalesced_events()
    }
}
